//! SUNAT REST API used for guias de remision (GRE): an OAuth token, then the
//! zipped document is sent and its ticket checked with that token.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::StatusCode;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::constants::HTTP_CLIENT;
use crate::fault::SunatFault;
use crate::models::Credentials;
use crate::sunat_client::{
    send_request, File, SendFileResponse, SunatClientError, VerifyTicketResponse,
    VerifyTicketStatus,
};

const TOKEN_URL: &str = "https://api-seguridad.sunat.gob.pe/v1/clientessol";
const TOKEN_SCOPE: &str = "https://api-cpe.sunat.gob.pe";

/// POST `{url}/comprobantes/{name}`. SUNAT answers with a ticket.
pub async fn send_document(
    url: &str,
    file: &File,
    credentials: &Credentials,
) -> Result<SendFileResponse, SunatClientError> {
    let token = fetch_token(credentials).await?;
    let name = file.filename.strip_suffix(".zip").unwrap_or(&file.filename);
    let payload = json!({
        "archivo": {
            "nomArchivo": file.filename,
            "arcGreZip": file.zip_base64,
            "hashZip": hash_zip(&file.zip_base64)?,
        }
    })
    .to_string();

    let (status, body) = send_request(|| {
        HTTP_CLIENT
            .post(format!("{url}/comprobantes/{name}"))
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .body(payload.clone())
    })
    .await?;
    forget_rejected_token(status, credentials);
    parse_send_response(status, &body)
}

/// GET `{url}/comprobantes/envios/{ticket}`
pub async fn verify_ticket(
    url: &str,
    ticket: &str,
    credentials: &Credentials,
) -> Result<VerifyTicketResponse, SunatClientError> {
    let token = fetch_token(credentials).await?;
    let (status, body) = send_request(|| {
        HTTP_CLIENT
            .get(format!("{url}/comprobantes/envios/{ticket}"))
            .bearer_auth(&token)
    })
    .await?;
    forget_rejected_token(status, credentials);
    parse_ticket_response(status, &body)
}

/// Access tokens by client id and SOL user, with the instant they stop being reused
static TOKENS: LazyLock<Mutex<HashMap<String, (String, Instant)>>> =
    LazyLock::new(Default::default);

/// Requests an access token with the SOL user and the API client id/secret,
/// reusing the previous one until a minute before it expires (or until SUNAT
/// rejects it).
async fn fetch_token(credentials: &Credentials) -> Result<String, SunatClientError> {
    let (Some(client_id), Some(client_secret)) =
        (&credentials.client_id, &credentials.client_secret)
    else {
        return Err(
            anyhow!("client_id and client_secret are required for guias de remision").into(),
        );
    };

    let key = token_key(client_id, &credentials.username);
    if let Some(token) = cached_token(&key) {
        return Ok(token);
    }

    let (status, body) = send_request(|| {
        HTTP_CLIENT
            .post(format!("{TOKEN_URL}/{client_id}/oauth2/token/"))
            .form(&[
                ("grant_type", "password"),
                ("scope", TOKEN_SCOPE),
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("username", &credentials.username),
                ("password", &credentials.password),
            ])
    })
    .await?;

    let Some(json) = parse_json(&body) else {
        return Err(SunatClientError::Response(status));
    };
    let token = text(&json["access_token"])
        .ok_or_else(|| anyhow!("Could not get a SUNAT access token ({status}): {json}"))?;
    let expires_in = json["expires_in"].as_u64().unwrap_or(3600);
    store_token(key, &token, expires_in);
    Ok(token)
}

fn token_key(client_id: &str, username: &str) -> String {
    format!("{client_id}:{username}")
}

/// A token SUNAT rejects (e.g. revoked before it expires) is not reused
fn forget_rejected_token(status: StatusCode, credentials: &Credentials) {
    if status != StatusCode::UNAUTHORIZED {
        return;
    }
    if let (Some(client_id), Ok(mut tokens)) = (&credentials.client_id, TOKENS.lock()) {
        tokens.remove(&token_key(client_id, &credentials.username));
    }
}

fn cached_token(key: &str) -> Option<String> {
    let tokens = TOKENS.lock().ok()?;
    let (token, reuse_until) = tokens.get(key)?;
    (Instant::now() < *reuse_until).then(|| token.clone())
}

fn store_token(key: String, token: &str, expires_in: u64) {
    let reuse_until = Instant::now() + Duration::from_secs(expires_in.saturating_sub(60));
    if let Ok(mut tokens) = TOKENS.lock() {
        tokens.insert(key, (token.to_string(), reuse_until));
    }
}

/// Hex SHA-256 of the zip, as SUNAT expects in `hashZip`
fn hash_zip(zip_base64: &str) -> Result<String, SunatClientError> {
    let zip = general_purpose::STANDARD
        .decode(zip_base64)
        .map_err(|e| anyhow!("Invalid zip: {e}"))?;
    Ok(format!("{:x}", Sha256::digest(zip)))
}

fn parse_send_response(
    status: StatusCode,
    body: &str,
) -> Result<SendFileResponse, SunatClientError> {
    let Some(json) = parse_json(body) else {
        return Err(SunatClientError::Response(status));
    };
    if let Some(ticket) = text(&json["numTicket"]) {
        return Ok(SendFileResponse::Ticket(ticket));
    }
    rest_error(&json)
        .map(SendFileResponse::Fault)
        .ok_or(SunatClientError::Response(status))
}

fn parse_ticket_response(
    status: StatusCode,
    body: &str,
) -> Result<VerifyTicketResponse, SunatClientError> {
    let Some(json) = parse_json(body) else {
        return Err(SunatClientError::Response(status));
    };
    if let Some(status_code) = text(&json["codRespuesta"]) {
        if let Some(cdr_base64) = text(&json["arcCdr"]) {
            return Ok(VerifyTicketResponse::Cdr(VerifyTicketStatus {
                cdr_base64,
                status_code,
            }));
        }
        if let Some(code) = text(&json["error"]["numError"]) {
            let message = text(&json["error"]["desError"]).unwrap_or_default();
            return Ok(VerifyTicketResponse::Fault(SunatFault { code, message }));
        }
        return Ok(VerifyTicketResponse::NoCdr { status_code });
    }
    rest_error(&json)
        .map(VerifyTicketResponse::Fault)
        .ok_or(SunatClientError::Response(status))
}

/// SUNAT's REST error body: `{cod|status, msg|message, errors: [{cod, msg}]}`.
/// The detailed errors are appended to the message as "cod - msg".
fn rest_error(json: &Value) -> Option<SunatFault> {
    let code = text(&json["cod"]).or_else(|| text(&json["status"]))?;
    let mut parts: Vec<String> = text(&json["msg"])
        .or_else(|| text(&json["message"]))
        .into_iter()
        .collect();
    parts.extend(
        json["errors"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|error| {
                format!(
                    "{} - {}",
                    text(&error["cod"]).unwrap_or_default(),
                    text(&error["msg"]).unwrap_or_default()
                )
            }),
    );
    Some(SunatFault {
        code,
        message: parts.join("; "),
    })
}

fn parse_json(body: &str) -> Option<Value> {
    serde_json::from_str(body).ok()
}

/// A non-empty string or number field as text
fn text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) if !text.is_empty() => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use super::{
        cached_token, forget_rejected_token, hash_zip, parse_send_response, parse_ticket_response,
        store_token,
    };
    use crate::models::Credentials;
    use crate::models::Status;
    use crate::sunat_client::{SendFileResponse, VerifyTicketResponse};

    #[test]
    fn hash_zip_is_hex_sha256_of_the_zip_bytes() {
        // base64("abc")
        assert_eq!(
            hash_zip("YWJj").unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn send_returns_the_ticket() {
        let body = r#"{"numTicket":"a7b3c1d2-0001","fecRecepcion":"2023-01-01T10:00:00"}"#;
        let response = parse_send_response(StatusCode::OK, body).unwrap();
        assert!(matches!(response, SendFileResponse::Ticket(ticket) if ticket == "a7b3c1d2-0001"));
    }

    #[test]
    fn send_returns_sunat_errors() {
        let body = r#"{"cod":"422","msg":"Unprocessable Entity","errors":[{"cod":"1033","msg":"El comprobante fue registrado previamente"}]}"#;
        let response = parse_send_response(StatusCode::UNPROCESSABLE_ENTITY, body).unwrap();
        let SendFileResponse::Fault(fault) = response else {
            panic!("Expected a fault");
        };
        assert_eq!(fault.code, "422");
        assert_eq!(
            fault.message,
            "Unprocessable Entity; 1033 - El comprobante fue registrado previamente"
        );
    }

    #[test]
    fn send_without_json_body_is_an_error() {
        assert!(parse_send_response(StatusCode::BAD_GATEWAY, "<html/>").is_err());
    }

    #[test]
    fn ticket_with_cdr() {
        let body = r#"{"codRespuesta":"0","indCdrGenerado":"1","arcCdr":"UEsDBA=="}"#;
        let response = parse_ticket_response(StatusCode::OK, body).unwrap();
        let VerifyTicketResponse::Cdr(status) = response else {
            panic!("Expected a CDR");
        };
        assert_eq!(status.cdr_base64, "UEsDBA==");
        assert_eq!(status.status_code, "0");
    }

    #[test]
    fn ticket_in_progress() {
        let body = r#"{"codRespuesta":"98"}"#;
        let response = parse_ticket_response(StatusCode::OK, body).unwrap();
        assert!(
            matches!(response, VerifyTicketResponse::NoCdr { status_code } if status_code == "98")
        );
    }

    #[test]
    fn ticket_with_error_and_no_cdr() {
        let body = r#"{"codRespuesta":"99","indCdrGenerado":"0","error":{"numError":"2223","desError":"El documento ya fue informado"}}"#;
        let response = parse_ticket_response(StatusCode::OK, body).unwrap();
        let VerifyTicketResponse::Fault(fault) = response else {
            panic!("Expected a fault");
        };
        assert_eq!(fault.code, "2223");
        assert_eq!(fault.message, "El documento ya fue informado");
        assert_eq!(fault.status(), Status::Rejected);
    }

    #[test]
    fn ticket_not_found() {
        let body = r#"{"status":404,"message":"Ticket no encontrado"}"#;
        let response = parse_ticket_response(StatusCode::NOT_FOUND, body).unwrap();
        assert!(matches!(response, VerifyTicketResponse::Fault(fault) if fault.code == "404"));
    }

    #[test]
    fn tokens_are_reused_until_a_minute_before_they_expire() {
        store_token("client-a:user".into(), "token-a", 3600);
        assert_eq!(cached_token("client-a:user").as_deref(), Some("token-a"));

        // Less than a minute left: request a new one
        store_token("client-b:user".into(), "token-b", 30);
        assert_eq!(cached_token("client-b:user"), None);

        assert_eq!(cached_token("unknown"), None);

        // A rejected token is dropped
        store_token("client-c:user".into(), "token-c", 3600);
        let credentials = Credentials {
            username: "user".into(),
            client_id: Some("client-c".into()),
            ..Default::default()
        };
        forget_rejected_token(StatusCode::OK, &credentials);
        assert_eq!(cached_token("client-c:user").as_deref(), Some("token-c"));
        forget_rejected_token(StatusCode::UNAUTHORIZED, &credentials);
        assert_eq!(cached_token("client-c:user"), None);
    }
}
