//! SUNAT REST API used for guias de remision (GRE): an OAuth token, then the
//! zipped document is sent and its ticket checked with that token.

use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::StatusCode;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::client_sunat::{
    ClientSunatErr, ErrorResponse, File, SendFileResponse, VerifyTicketResponse, VerifyTicketStatus,
};
use crate::constants::HTTP_CLIENT;
use crate::models::Credentials;

const TOKEN_URL: &str = "https://api-seguridad.sunat.gob.pe/v1/clientessol";
const TOKEN_SCOPE: &str = "https://api-cpe.sunat.gob.pe";

/// POST `{url}/comprobantes/{name}`. SUNAT answers with a ticket.
pub async fn send_document(
    url: &str,
    file: &File,
    credentials: &Credentials,
) -> Result<SendFileResponse, ClientSunatErr> {
    let token = fetch_token(credentials).await?;
    let name = file.name.trim_end_matches(".zip");
    let payload = json!({
        "archivo": {
            "nomArchivo": file.name,
            "arcGreZip": file.base_64,
            "hashZip": hash_zip(&file.base_64)?,
        }
    });

    let response = HTTP_CLIENT
        .post(format!("{url}/comprobantes/{name}"))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;
    let status = response.status();
    parse_send_response(status, &response.text().await?)
}

/// GET `{url}/comprobantes/envios/{ticket}`
pub async fn verify_ticket(
    url: &str,
    ticket: &str,
    credentials: &Credentials,
) -> Result<VerifyTicketResponse, ClientSunatErr> {
    let token = fetch_token(credentials).await?;
    let response = HTTP_CLIENT
        .get(format!("{url}/comprobantes/envios/{ticket}"))
        .bearer_auth(token)
        .send()
        .await?;
    let status = response.status();
    parse_ticket_response(status, &response.text().await?)
}

/// Requests an access token with the SOL user and the API client id/secret.
/// A new token is requested for every call; it is valid for about an hour.
async fn fetch_token(credentials: &Credentials) -> Result<String, ClientSunatErr> {
    let (Some(client_id), Some(client_secret)) =
        (&credentials.client_id, &credentials.client_secret)
    else {
        return Err(
            anyhow!("client_id and client_secret are required for guias de remision").into(),
        );
    };

    let response = HTTP_CLIENT
        .post(format!("{TOKEN_URL}/{client_id}/oauth2/token/"))
        .form(&[
            ("grant_type", "password"),
            ("scope", TOKEN_SCOPE),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("username", &credentials.username),
            ("password", &credentials.password),
        ])
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;

    parse_json(&body)
        .and_then(|json| text(&json["access_token"]))
        .ok_or_else(|| anyhow!("Could not get a SUNAT access token ({status}): {body}").into())
}

/// Hex SHA-256 of the zip, as SUNAT expects in `hashZip`
fn hash_zip(zip_base64: &str) -> Result<String, ClientSunatErr> {
    let zip = general_purpose::STANDARD
        .decode(zip_base64)
        .map_err(|e| anyhow!("Invalid zip: {e}"))?;
    Ok(format!("{:x}", Sha256::digest(zip)))
}

fn parse_send_response(status: StatusCode, body: &str) -> Result<SendFileResponse, ClientSunatErr> {
    let json = parse_json(body);
    if let Some(ticket) = json.as_ref().and_then(|json| text(&json["numTicket"])) {
        return Ok(SendFileResponse::Ticket(ticket));
    }
    match json.as_ref().and_then(rest_error) {
        Some(error) => Ok(SendFileResponse::Error(error)),
        None => Err(ClientSunatErr::Response(status)),
    }
}

fn parse_ticket_response(
    status: StatusCode,
    body: &str,
) -> Result<VerifyTicketResponse, ClientSunatErr> {
    let Some(json) = parse_json(body) else {
        return Err(ClientSunatErr::Response(status));
    };

    if let Some(status_code) = text(&json["codRespuesta"]) {
        return Ok(match text(&json["arcCdr"]) {
            Some(cdr_base64) => VerifyTicketResponse::Cdr(VerifyTicketStatus {
                cdr_base64,
                status_code,
            }),
            None => match text(&json["error"]["numError"]) {
                Some(code) => VerifyTicketResponse::Error(ErrorResponse {
                    code,
                    message: text(&json["error"]["desError"]).unwrap_or_default(),
                }),
                None => VerifyTicketResponse::NoCdr { status_code },
            },
        });
    }

    match rest_error(&json) {
        Some(error) => Ok(VerifyTicketResponse::Error(error)),
        None => Err(ClientSunatErr::Response(status)),
    }
}

/// SUNAT's REST error body: `{cod|status, msg|message, errors: [{cod, msg}]}`.
/// The detailed errors are appended to the message as "cod - msg".
fn rest_error(json: &Value) -> Option<ErrorResponse> {
    let code = text(&json["cod"]).or_else(|| text(&json["status"]))?;
    let mut message = text(&json["msg"])
        .or_else(|| text(&json["message"]))
        .unwrap_or_default();
    for error in json["errors"].as_array().into_iter().flatten() {
        let detail = format!(
            "{} - {}",
            text(&error["cod"]).unwrap_or_default(),
            text(&error["msg"]).unwrap_or_default()
        );
        message = if message.is_empty() {
            detail
        } else {
            format!("{message}; {detail}")
        };
    }
    Some(ErrorResponse { code, message })
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

    use super::{hash_zip, parse_send_response, parse_ticket_response};
    use crate::client_sunat::{SendFileResponse, VerifyTicketResponse};
    use crate::models::Status;

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
        let SendFileResponse::Error(error) = response else {
            panic!("Expected an error");
        };
        assert_eq!(error.code, "422");
        assert_eq!(
            error.message,
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
            matches!(response, VerifyTicketResponse::NoCdr { status_code } if Status::from_code_str(&status_code) == Status::EnProceso)
        );
    }

    #[test]
    fn ticket_with_error_and_no_cdr() {
        let body = r#"{"codRespuesta":"99","indCdrGenerado":"0","error":{"numError":"2223","desError":"El documento ya fue informado"}}"#;
        let response = parse_ticket_response(StatusCode::OK, body).unwrap();
        let VerifyTicketResponse::Error(error) = response else {
            panic!("Expected an error");
        };
        assert_eq!(error.code, "2223");
        assert_eq!(error.message, "El documento ya fue informado");
        assert_eq!(error.status(), Status::Rechazado);
    }

    #[test]
    fn ticket_not_found() {
        let body = r#"{"status":404,"message":"Ticket no encontrado"}"#;
        let response = parse_ticket_response(StatusCode::NOT_FOUND, body).unwrap();
        assert!(matches!(response, VerifyTicketResponse::Error(error) if error.code == "404"));
    }
}
