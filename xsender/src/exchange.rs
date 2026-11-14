//! SUNAT calls without IO: each [`Call`] builds its HTTP request and parses
//! the answer, without sending anything. [`SunatClient`](crate::SunatClient)
//! (feature `http`) sends them with reqwest; the Go, Python and Java bindings
//! with their own HTTP clients.

use std::str::FromStr;

use serde::Serialize;

use crate::fault::SunatFault;
use crate::models::{
    ConsultDocument, Credentials, SendFileTarget, SoapFileTargetAction, ValidateCdp,
    VerifyTicketTarget,
};
use crate::rest;
use crate::soap::envelope::{BodyData, EnvelopeData, FileData, ToXmlString};
use crate::soap::send_file_response::SendFileXmlResponse;
use crate::soap::status_response::StatusXmlResponse;

/// A zip file for SUNAT
#[derive(Clone, Debug)]
pub struct File {
    pub filename: String,
    pub zip_base64: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SunatClientError {
    #[cfg(feature = "http")]
    #[error("Http client error")]
    HttpClient(#[from] reqwest::Error),
    /// The answer could not be parsed; the HTTP status says why, e.g. 503
    #[error("Unexpected response from server (HTTP {0})")]
    Response(u16),
    #[error("Error while creating the template")]
    Templating(#[from] tera::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub enum SendFileResponse {
    Cdr(String),
    Ticket(String),
    Fault(SunatFault),
}

pub enum VerifyTicketResponse {
    Cdr(VerifyTicketStatus),
    /// The ticket answer carries no CDR, e.g. status code 98 (still processing)
    NoCdr {
        status_code: String,
    },
    Fault(SunatFault),
}

/// Answer of the consult (BillConsultService) and validation (BillValidService) services
pub enum ConsultResponse {
    Status(ConsultStatus),
    Fault(SunatFault),
}

pub struct ConsultStatus {
    /// e.g. "0001" (accepted) for getStatus, or "0" for validaCDPcriterios
    pub status_code: String,
    pub status_message: Option<String>,
    /// CDR zip in base64 (getStatusCdr)
    pub cdr_base64: Option<String>,
}

pub struct VerifyTicketStatus {
    pub cdr_base64: String,
    pub status_code: String,
}

/// OAuth token of the REST API (guias de remision)
pub struct Token {
    pub access_token: String,
    /// Seconds
    pub expires_in: u64,
}

/// An HTTP request to send as is
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
}

/// A call to a SUNAT service
pub enum Call {
    SendFile {
        target: SendFileTarget,
        file: File,
    },
    VerifyTicket {
        target: VerifyTicketTarget,
        ticket: String,
    },
    /// BillConsultService getStatus: status of a document already sent
    GetStatus {
        url: String,
        document: ConsultDocument,
    },
    /// BillConsultService getStatusCdr: CDR of a document already sent
    GetStatusCdr {
        url: String,
        document: ConsultDocument,
    },
    /// BillValidService validaCDPcriterios
    ValidateCdp {
        url: String,
        cdp: ValidateCdp,
    },
    /// BillValidService verificaCPEarchivo
    ValidateFile {
        url: String,
        file: File,
    },
    /// OAuth token of the REST API (guias de remision)
    Token,
}

/// The parsed answer of a [`Call`], of the variant of the call
pub enum Answer {
    SendFile(SendFileResponse),
    VerifyTicket(VerifyTicketResponse),
    Consult(ConsultResponse),
    Token(Token),
}

impl Call {
    /// The REST API of the guias de remision needs a [`Call::Token`] first
    pub fn needs_token(&self) -> bool {
        matches!(
            self,
            Call::SendFile {
                target: SendFileTarget::Rest(..),
                ..
            } | Call::VerifyTicket {
                target: VerifyTicketTarget::Rest(_),
                ..
            }
        )
    }

    /// The HTTP request of the call; `token` is required when [`Self::needs_token`].
    pub fn request(
        &self,
        credentials: &Credentials,
        token: Option<&str>,
    ) -> Result<HttpRequest, SunatClientError> {
        match self {
            Call::SendFile {
                target: SendFileTarget::Soap(url, action),
                file,
            } => {
                let operation = match action {
                    SoapFileTargetAction::Bill => "sendBill",
                    SoapFileTargetAction::Summary => "sendSummary",
                    SoapFileTargetAction::Pack => "sendPack",
                };
                let body = BodyData::SendFile {
                    operation,
                    file: file_data(file),
                };
                soap_request(url, operation, credentials, body)
            }
            Call::SendFile {
                target: SendFileTarget::Rest(url, _),
                file,
            } => Ok(rest::send_document_request(url, file, required(token)?)),
            Call::VerifyTicket {
                target: VerifyTicketTarget::Soap(url),
                ticket,
            } => soap_request(
                url,
                "getStatus",
                credentials,
                BodyData::VerifyTicket(ticket.clone()),
            ),
            Call::VerifyTicket {
                target: VerifyTicketTarget::Rest(url),
                ticket,
            } => Ok(rest::verify_ticket_request(url, ticket, required(token)?)),
            Call::GetStatus { url, document } => {
                consult_request(url, "getStatus", credentials, document)
            }
            Call::GetStatusCdr { url, document } => {
                consult_request(url, "getStatusCdr", credentials, document)
            }
            Call::ValidateCdp { url, cdp } => soap_request(
                url,
                "validaCDPcriterios",
                credentials,
                BodyData::ValidateCdp(cdp.clone()),
            ),
            Call::ValidateFile { url, file } => soap_request(
                url,
                "verificaCPEarchivo",
                credentials,
                BodyData::ValidateFile(file_data(file)),
            ),
            Call::Token => rest::token_request(credentials),
        }
    }

    /// Parses the HTTP answer of [`Self::request`]. SUNAT answers SOAP faults
    /// with HTTP 500, so the body is parsed whatever the status.
    pub fn parse(&self, status: u16, body: &str) -> Result<Answer, SunatClientError> {
        match self {
            Call::SendFile {
                target: SendFileTarget::Soap(..),
                ..
            } => {
                parse_soap::<SendFileXmlResponse>(status, body).map(|r| Answer::SendFile(r.into()))
            }
            Call::SendFile {
                target: SendFileTarget::Rest(..),
                ..
            } => rest::parse_send_response(status, body).map(Answer::SendFile),
            Call::VerifyTicket {
                target: VerifyTicketTarget::Soap(_),
                ..
            } => parse_soap::<StatusXmlResponse>(status, body)
                .map(|r| Answer::VerifyTicket(r.into())),
            Call::VerifyTicket {
                target: VerifyTicketTarget::Rest(_),
                ..
            } => rest::parse_ticket_response(status, body).map(Answer::VerifyTicket),
            Call::GetStatus { .. }
            | Call::GetStatusCdr { .. }
            | Call::ValidateCdp { .. }
            | Call::ValidateFile { .. } => {
                parse_soap::<StatusXmlResponse>(status, body).map(|r| Answer::Consult(r.into()))
            }
            Call::Token => rest::parse_token(status, body).map(Answer::Token),
        }
    }
}

fn required(token: Option<&str>) -> Result<&str, SunatClientError> {
    token.ok_or_else(|| anyhow::anyhow!("A token is required for guias de remision").into())
}

fn file_data(file: &File) -> FileData {
    FileData {
        filename: file.filename.clone(),
        zip_base64: file.zip_base64.clone(),
    }
}

fn consult_request(
    url: &str,
    operation: &'static str,
    credentials: &Credentials,
    document: &ConsultDocument,
) -> Result<HttpRequest, SunatClientError> {
    let body = BodyData::ConsultDocument {
        operation,
        document: document.clone(),
    };
    soap_request(url, operation, credentials, body)
}

fn soap_request(
    url: &str,
    operation: &str,
    credentials: &Credentials,
    body: BodyData,
) -> Result<HttpRequest, SunatClientError> {
    let envelope = EnvelopeData {
        username: credentials.username.clone(),
        password: credentials.password.clone(),
        body,
    };
    Ok(HttpRequest {
        method: HttpMethod::Post,
        url: url.to_string(),
        headers: vec![
            ("Content-Type".into(), "text/xml; charset=utf-8".into()),
            ("SOAPAction".into(), format!("urn:{operation}")),
        ],
        body: Some(envelope.to_xml_string()?),
    })
}

fn parse_soap<T>(status: u16, body: &str) -> Result<T, SunatClientError>
where
    T: FromStr<Err = anyhow::Error>,
{
    match T::from_str(body) {
        Ok(response) => Ok(response),
        Err(_) if !(200..300).contains(&status) => Err(SunatClientError::Response(status)),
        Err(error) => Err(error.into()),
    }
}

impl From<StatusXmlResponse> for ConsultResponse {
    fn from(value: StatusXmlResponse) -> Self {
        match value {
            StatusXmlResponse::Status(status) => Self::Status(ConsultStatus {
                status_code: status.status_code,
                status_message: status.status_message,
                cdr_base64: status.cdr_base64,
            }),
            StatusXmlResponse::Fault(fault) => Self::Fault(fault),
        }
    }
}

impl From<SendFileXmlResponse> for SendFileResponse {
    fn from(value: SendFileXmlResponse) -> Self {
        match value {
            SendFileXmlResponse::Cdr(cdr) => Self::Cdr(cdr),
            SendFileXmlResponse::Ticket(ticket) => Self::Ticket(ticket),
            SendFileXmlResponse::Fault(fault) => Self::Fault(fault),
        }
    }
}

impl From<StatusXmlResponse> for VerifyTicketResponse {
    fn from(value: StatusXmlResponse) -> Self {
        match value {
            StatusXmlResponse::Status(status) => match status.cdr_base64 {
                Some(cdr_base64) => Self::Cdr(VerifyTicketStatus {
                    cdr_base64,
                    status_code: status.status_code,
                }),
                None => Self::NoCdr {
                    status_code: status.status_code,
                },
            },
            StatusXmlResponse::Fault(fault) => Self::Fault(fault),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{Answer, Call, File, HttpMethod, SendFileResponse, SunatClientError};
    use crate::models::{Credentials, SendFileTarget, SoapFileTargetAction, Status};

    const RESOURCES: &str = "resources/test";

    fn send_bill() -> Call {
        Call::SendFile {
            target: SendFileTarget::Soap(
                "https://sunat/billService".into(),
                SoapFileTargetAction::Bill,
            ),
            file: File {
                filename: "F001-1.zip".into(),
                zip_base64: "UEsDBA==".into(),
            },
        }
    }

    #[test]
    fn soap_request_carries_operation_and_credentials() {
        let credentials = Credentials {
            username: "12345678959MODDATOS".into(),
            password: "MODDATOS".into(),
            ..Default::default()
        };
        let request = send_bill().request(&credentials, None).unwrap();

        assert_eq!(request.method, HttpMethod::Post);
        assert_eq!(request.url, "https://sunat/billService");
        assert!(request
            .headers
            .contains(&("SOAPAction".into(), "urn:sendBill".into())));
        let body = request.body.unwrap();
        assert!(body.contains("<wsse:Username>12345678959MODDATOS</wsse:Username>"));
        assert!(body.contains("<ns2:sendBill"));
    }

    #[test]
    fn rest_calls_need_a_token() {
        let call = Call::VerifyTicket {
            target: crate::models::VerifyTicketTarget::Rest("https://api".into()),
            ticket: "123".into(),
        };
        assert!(call.needs_token());
        assert!(call.request(&Credentials::default(), None).is_err());
        assert!(!send_bill().needs_token());
    }

    #[test]
    fn http_500_with_soap_fault_is_parsed() {
        let body = fs::read_to_string(format!(
            "{RESOURCES}/bill_service_response_fault_rechazo.xml"
        ))
        .expect("Could not read file");
        let Ok(Answer::SendFile(SendFileResponse::Fault(fault))) = send_bill().parse(500, &body)
        else {
            panic!("A SOAP fault must be returned as a response");
        };
        assert_eq!(fault.sunat_code(), Some(2335));
        assert_eq!(fault.status(), Status::Rejected);
    }

    #[test]
    fn http_error_without_soap_body_is_an_error() {
        let result = send_bill().parse(503, "<html><body>Service Unavailable</body></html>");
        assert!(matches!(result, Err(SunatClientError::Response(503))));
    }
}
