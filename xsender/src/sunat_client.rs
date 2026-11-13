use std::str::FromStr;

use reqwest::StatusCode;

use crate::constants::HTTP_CLIENT;
use crate::models::{
    ConsultDocument, Credentials, SendFileTarget, SoapFileTargetAction, ValidateCdp,
    VerifyTicketTarget,
};
use crate::rest;
use crate::soap::envelope::{BodyData, EnvelopeData, FileData, ToStringXml};
use crate::soap::send_file_response::SendFileXmlResponse;
use crate::soap::status_response::StatusXmlResponse;

#[derive(Default)]
pub struct SunatClient {}

pub struct File {
    pub name: String,
    pub base_64: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientSunatErr {
    #[error("Http client error")]
    HttpClient(reqwest::Error),
    #[error("Unexpected response from server")]
    Response(StatusCode),
    #[error("Error while creating the template")]
    Templating(tera::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<reqwest::Error> for ClientSunatErr {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpClient(e)
    }
}

impl From<tera::Error> for ClientSunatErr {
    fn from(e: tera::Error) -> Self {
        Self::Templating(e)
    }
}

pub enum SendFileResponse {
    Cdr(String),
    Ticket(String),
    Error(ErrorResponse),
}

pub enum VerifyTicketResponse {
    Cdr(VerifyTicketStatus),
    /// The ticket answer carries no CDR, e.g. status code 98 (still processing)
    NoCdr {
        status_code: String,
    },
    Error(ErrorResponse),
}

/// Answer of the consult (BillConsultService) and validation (BillValidService) services
pub enum ConsultResponse {
    Status(ConsultStatus),
    Error(ErrorResponse),
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

/// Error returned by SUNAT (SOAP fault or REST error)
pub use crate::soap::SoapFault as ErrorResponse;

impl SunatClient {
    pub async fn send_file(
        &self,
        target: &SendFileTarget,
        file: &File,
        credentials: &Credentials,
    ) -> Result<SendFileResponse, ClientSunatErr> {
        match &target {
            SendFileTarget::Soap(url, action) => {
                let operation = match action {
                    SoapFileTargetAction::Bill => "sendBill",
                    SoapFileTargetAction::Summary => "sendSummary",
                    SoapFileTargetAction::Pack => "sendPack",
                };

                let envelope = EnvelopeData {
                    username: credentials.username.clone(),
                    password: credentials.password.clone(),
                    body: BodyData::SendFile {
                        operation,
                        file: FileData {
                            filename: file.name.clone(),
                            file_content: file.base_64.clone(),
                        },
                    },
                };

                let body = envelope.to_string_xml()?;
                let soap_action = format!("urn:{operation}");
                let (status, response_body) = post_soap(url, body, &soap_action).await?;
                parse_soap_response::<SendFileXmlResponse>(status, &response_body).map(Into::into)
            }
            SendFileTarget::Rest(url, _) => rest::send_document(url, file, credentials).await,
        }
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
        credentials: &Credentials,
    ) -> Result<VerifyTicketResponse, ClientSunatErr> {
        match &target {
            VerifyTicketTarget::Soap(url) => {
                let envelope = EnvelopeData {
                    username: credentials.username.clone(),
                    password: credentials.password.clone(),
                    body: BodyData::VerifyTicket(ticket.to_string()),
                };

                let body = envelope.to_string_xml()?;
                let (status, response_body) = post_soap(url, body, "urn:getStatus").await?;
                parse_soap_response::<StatusXmlResponse>(status, &response_body).map(Into::into)
            }
            VerifyTicketTarget::Rest(url) => rest::verify_ticket(url, ticket, credentials).await,
        }
    }

    /// BillConsultService getStatus: status of a document already sent
    pub async fn get_status(
        &self,
        url: &str,
        document: &ConsultDocument,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, ClientSunatErr> {
        let operation = "getStatus";
        let body = BodyData::ConsultDocument {
            operation,
            document: document.clone(),
        };
        consult(url, body, operation, credentials).await
    }

    /// BillConsultService getStatusCdr: status and CDR of a document already sent
    pub async fn get_status_cdr(
        &self,
        url: &str,
        document: &ConsultDocument,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, ClientSunatErr> {
        let operation = "getStatusCdr";
        let body = BodyData::ConsultDocument {
            operation,
            document: document.clone(),
        };
        consult(url, body, operation, credentials).await
    }

    /// BillValidService validaCDPcriterios: checks a document by its data
    pub async fn validate_cdp(
        &self,
        url: &str,
        cdp: &ValidateCdp,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, ClientSunatErr> {
        let body = BodyData::ValidateCdp(cdp.clone());
        consult(url, body, "validaCDPcriterios", credentials).await
    }

    /// BillValidService verificaCPEarchivo: checks a signed XML file
    pub async fn validate_file(
        &self,
        url: &str,
        file: &File,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, ClientSunatErr> {
        let body = BodyData::ValidateFile(FileData {
            filename: file.name.clone(),
            file_content: file.base_64.clone(),
        });
        consult(url, body, "verificaCPEarchivo", credentials).await
    }
}

async fn consult(
    url: &str,
    body: BodyData,
    operation: &str,
    credentials: &Credentials,
) -> Result<ConsultResponse, ClientSunatErr> {
    let envelope = EnvelopeData {
        username: credentials.username.clone(),
        password: credentials.password.clone(),
        body,
    };
    let (status, response_body) =
        post_soap(url, envelope.to_string_xml()?, &format!("urn:{operation}")).await?;
    parse_soap_response::<StatusXmlResponse>(status, &response_body).map(Into::into)
}

impl From<StatusXmlResponse> for ConsultResponse {
    fn from(value: StatusXmlResponse) -> Self {
        match value {
            StatusXmlResponse::Status(status) => Self::Status(ConsultStatus {
                status_code: status.status_code,
                status_message: status.status_message,
                cdr_base64: status.cdr_base64,
            }),
            StatusXmlResponse::Fault(fault) => Self::Error(fault),
        }
    }
}

impl From<SendFileXmlResponse> for SendFileResponse {
    fn from(value: SendFileXmlResponse) -> Self {
        match value {
            SendFileXmlResponse::Cdr(cdr) => Self::Cdr(cdr),
            SendFileXmlResponse::Ticket(ticket) => Self::Ticket(ticket),
            SendFileXmlResponse::Fault(fault) => Self::Error(fault),
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
            StatusXmlResponse::Fault(fault) => Self::Error(fault),
        }
    }
}

/// Sends a request and returns the HTTP status and body. A connection failure
/// is retried once; that request never reached SUNAT, so retrying it can't
/// register the same document twice.
pub(crate) async fn send_request(
    request: impl Fn() -> reqwest::RequestBuilder,
) -> Result<(StatusCode, String), ClientSunatErr> {
    let response = match request().send().await {
        Err(error) if error.is_connect() => request().send().await?,
        other => other?,
    };
    let status = response.status();
    Ok((status, response.text().await?))
}

/// POSTs a SOAP envelope
async fn post_soap(
    url: &str,
    body: String,
    soap_action: &str,
) -> Result<(StatusCode, String), ClientSunatErr> {
    send_request(|| {
        HTTP_CLIENT
            .post(url)
            .body(body.clone())
            .header("Content-Type", "text/xml; charset=utf-8")
            .header("SOAPAction", soap_action)
    })
    .await
}

/// SUNAT reports rejections as SOAP faults with HTTP 500, so the body is
/// parsed regardless of the status code. A non-2xx response whose body is not
/// a SOAP response is returned as [`ClientSunatErr::Response`].
fn parse_soap_response<T>(status: StatusCode, body: &str) -> Result<T, ClientSunatErr>
where
    T: FromStr<Err = anyhow::Error>,
{
    match T::from_str(body) {
        Ok(response) => Ok(response),
        Err(_) if !status.is_success() => Err(ClientSunatErr::Response(status)),
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use reqwest::StatusCode;

    use super::{parse_soap_response, ClientSunatErr, SendFileResponse};
    use crate::models::Status;
    use crate::soap::send_file_response::SendFileXmlResponse;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn http_500_with_soap_fault_is_parsed() {
        let body = fs::read_to_string(format!(
            "{RESOURCES}/bill_service_response_fault_rechazo.xml"
        ))
        .expect("Could not read file");
        let response: SendFileResponse =
            parse_soap_response::<SendFileXmlResponse>(StatusCode::INTERNAL_SERVER_ERROR, &body)
                .map(Into::into)
                .expect("A SOAP fault must be returned as a response");

        let SendFileResponse::Error(error) = response else {
            panic!("Expected an error response");
        };
        assert_eq!(error.sunat_code(), Some(2335));
        assert_eq!(error.status(), Status::Rechazado);
    }

    #[test]
    fn http_error_without_soap_body_is_an_error() {
        let result = parse_soap_response::<SendFileXmlResponse>(
            StatusCode::SERVICE_UNAVAILABLE,
            "<html><body>Service Unavailable</body></html>",
        );
        assert!(matches!(
            result,
            Err(ClientSunatErr::Response(StatusCode::SERVICE_UNAVAILABLE))
        ));
    }
}
