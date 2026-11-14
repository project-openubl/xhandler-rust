//! SUNAT REST API used for guias de remision (GRE): an OAuth token, then the
//! zipped document is sent and its ticket checked with that token. Requests
//! are built and answers parsed here; see [`crate::exchange`].

use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize;
use serde_json::Value;

use crate::exchange::{
    File, HttpMethod, HttpRequest, SendFileResponse, SunatClientError, Token, VerifyTicketResponse,
    VerifyTicketStatus,
};
use crate::fault::SunatFault;
use crate::models::Credentials;
use crate::ubl_file::sha256_hex;

/// Where SUNAT gives the OAuth tokens
pub const SECURITY_API_URL: &str = "https://api-seguridad.sunat.gob.pe";
const TOKEN_SCOPE: &str = "https://api-cpe.sunat.gob.pe";

pub fn token_request(
    security_api_url: &str,
    credentials: &Credentials,
) -> Result<HttpRequest, SunatClientError> {
    let (Some(client_id), Some(client_secret)) =
        (&credentials.client_id, &credentials.client_secret)
    else {
        return Err(
            anyhow!("client_id and client_secret are required for guias de remision").into(),
        );
    };
    let form = serde_urlencoded::to_string([
        ("grant_type", "password"),
        ("scope", TOKEN_SCOPE),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("username", &credentials.username),
        ("password", &credentials.password),
    ])
    .map_err(|e| anyhow!("Invalid credentials: {e}"))?;
    Ok(HttpRequest {
        method: HttpMethod::Post,
        url: format!(
            "{security_api_url}/v1/clientessol/{}/oauth2/token/",
            path_segment(client_id)
        ),
        headers: vec![(
            "Content-Type".into(),
            "application/x-www-form-urlencoded".into(),
        )],
        body: Some(form),
    })
}

pub fn parse_token(status: u16, body: &str) -> Result<Token, SunatClientError> {
    let Some(json) = parse_json(body) else {
        return Err(SunatClientError::Response(status));
    };
    let access_token = text(&json["access_token"])
        .ok_or_else(|| anyhow!("Could not get a SUNAT access token ({status}): {json}"))?;
    let expires_in = json["expires_in"].as_u64().unwrap_or(3600);
    Ok(Token {
        access_token,
        expires_in,
    })
}

/// Body of the send request. A struct, so the bytes don't depend on whether
/// serde_json preserves the order of maps (a crate feature).
#[derive(Serialize)]
struct SendDocument<'a> {
    archivo: Archivo<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Archivo<'a> {
    arc_gre_zip: &'a str,
    hash_zip: String,
    nom_archivo: &'a str,
}

pub fn send_document_request(url: &str, file: &File, token: &str) -> HttpRequest {
    let name = file.filename.strip_suffix(".zip").unwrap_or(&file.filename);
    let payload = SendDocument {
        archivo: Archivo {
            arc_gre_zip: &file.zip_base64,
            hash_zip: hash_zip(&file.zip_base64),
            nom_archivo: &file.filename,
        },
    };
    HttpRequest {
        method: HttpMethod::Post,
        url: format!("{url}/comprobantes/{}", path_segment(name)),
        headers: vec![
            ("Authorization".into(), format!("Bearer {token}")),
            ("Content-Type".into(), "application/json".into()),
        ],
        body: Some(serde_json::to_string(&payload).expect("the body is serializable")),
    }
}

pub fn verify_ticket_request(url: &str, ticket: &str, token: &str) -> HttpRequest {
    HttpRequest {
        method: HttpMethod::Get,
        url: format!("{url}/comprobantes/envios/{}", path_segment(ticket)),
        headers: vec![("Authorization".into(), format!("Bearer {token}"))],
        body: None,
    }
}

/// Percent-encodes a value for a URL path: everything but A-Z a-z 0-9 - . _ ~
fn path_segment(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(byte).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

/// Hex SHA-256 of the zip bytes; the zip comes from this crate, so it is
/// valid base64.
fn hash_zip(zip_base64: &str) -> String {
    sha256_hex(
        &general_purpose::STANDARD
            .decode(zip_base64)
            .unwrap_or_default(),
    )
}

pub fn parse_send_response(status: u16, body: &str) -> Result<SendFileResponse, SunatClientError> {
    let Some(json) = parse_json(body).filter(|_| status < 500) else {
        return Err(SunatClientError::Response(status));
    };
    if let Some(ticket) = text(&json["numTicket"]) {
        return Ok(SendFileResponse::Ticket(ticket));
    }
    rest_error(&json)
        .map(SendFileResponse::Fault)
        .ok_or(SunatClientError::Response(status))
}

pub fn parse_ticket_response(
    status: u16,
    body: &str,
) -> Result<VerifyTicketResponse, SunatClientError> {
    // A server error (5xx) is not an answer about the document
    let Some(json) = parse_json(body).filter(|_| status < 500) else {
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
    use super::{hash_zip, parse_send_response, parse_ticket_response, parse_token, token_request};
    use crate::exchange::{SendFileResponse, VerifyTicketResponse};
    use crate::models::{Credentials, Status};

    #[test]
    fn hash_zip_is_hex_sha256_of_the_zip_bytes() {
        // base64("abc")
        assert_eq!(
            hash_zip("YWJj"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn send_returns_the_ticket() {
        let body = r#"{"numTicket":"a7b3c1d2-0001","fecRecepcion":"2023-01-01T10:00:00"}"#;
        let response = parse_send_response(200, body).unwrap();
        assert!(matches!(response, SendFileResponse::Ticket(ticket) if ticket == "a7b3c1d2-0001"));
    }

    #[test]
    fn send_returns_sunat_errors() {
        let body = r#"{"cod":"422","msg":"Unprocessable Entity","errors":[{"cod":"1033","msg":"El comprobante fue registrado previamente"}]}"#;
        let response = parse_send_response(422, body).unwrap();
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
        assert!(parse_send_response(502, "<html/>").is_err());
    }

    #[test]
    fn ticket_with_cdr() {
        let body = r#"{"codRespuesta":"0","indCdrGenerado":"1","arcCdr":"UEsDBA=="}"#;
        let response = parse_ticket_response(200, body).unwrap();
        let VerifyTicketResponse::Cdr(status) = response else {
            panic!("Expected a CDR");
        };
        assert_eq!(status.cdr_base64, "UEsDBA==");
        assert_eq!(status.status_code, "0");
    }

    #[test]
    fn ticket_in_progress() {
        let body = r#"{"codRespuesta":"98"}"#;
        let response = parse_ticket_response(200, body).unwrap();
        assert!(
            matches!(response, VerifyTicketResponse::NoCdr { status_code } if status_code == "98")
        );
    }

    #[test]
    fn ticket_with_error_and_no_cdr() {
        let body = r#"{"codRespuesta":"99","indCdrGenerado":"0","error":{"numError":"2223","desError":"El documento ya fue informado"}}"#;
        let response = parse_ticket_response(200, body).unwrap();
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
        let response = parse_ticket_response(404, body).unwrap();
        assert!(matches!(response, VerifyTicketResponse::Fault(fault) if fault.code == "404"));
    }

    #[test]
    fn token_request_is_a_password_grant() {
        let credentials = Credentials {
            username: "20123456789MODDATOS".into(),
            password: "pass&word".into(),
            client_id: Some("client".into()),
            client_secret: Some("secret".into()),
        };
        let request = token_request(super::SECURITY_API_URL, &credentials).unwrap();
        assert_eq!(
            request.url,
            "https://api-seguridad.sunat.gob.pe/v1/clientessol/client/oauth2/token/"
        );
        let body = request.body.unwrap();
        assert!(body.starts_with("grant_type=password&scope=https%3A%2F%2Fapi-cpe.sunat.gob.pe"));
        assert!(body.ends_with("&password=pass%26word"));

        assert!(token_request(super::SECURITY_API_URL, &Credentials::default()).is_err());
    }

    #[test]
    fn path_segments_are_percent_encoded() {
        assert_eq!(super::path_segment("T001-1 a/b"), "T001-1%20a%2Fb");
    }

    #[test]
    fn server_errors_are_not_answers() {
        assert!(parse_send_response(502, r#"{"cod":"502","msg":"Bad Gateway"}"#).is_err());
    }

    #[test]
    fn token_answer() {
        let token = parse_token(200, r#"{"access_token":"abc","expires_in":600}"#).unwrap();
        assert_eq!(
            (token.access_token.as_str(), token.expires_in),
            ("abc", 600)
        );
        assert!(parse_token(401, r#"{"error":"invalid_client"}"#).is_err());
        assert!(parse_token(502, "<html/>").is_err());
    }
}
