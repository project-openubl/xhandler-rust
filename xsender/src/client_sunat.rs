use std::str::FromStr;

use crate::constants::HTTP_CLIENT;
use crate::models::{Credentials, SendFileTarget, SoapFileTargetAction, VerifyTicketTarget};
use crate::soap::envelope::{BodyData, EnvelopeData, FileData, ToStringXml};
use crate::soap::send_file_response::{SendFileXmlResponse, SendFileXmlResponseFromStrError};
use crate::soap::verify_ticket_response::{
    VerifyTicketXmlResponse, VerifyTicketXmlResponseFromStrError,
};

#[derive(Default)]
pub struct ClientSUNAT {}

pub struct File {
    pub name: String,
    pub base_64: String,
}

#[derive(Debug)]
pub struct ErrorClientSUNAT {
    pub kind: Layer,
}

#[derive(Debug)]
pub enum Layer {
    Templating(String),
    Request(reqwest::Error),
    ReadingResponse,
    ServiceUnavailable(String),
}

pub enum SendFileResponse {
    Cdr(String),
    Ticket(String),
    Error(ErrorResponse),
}

pub enum VerifyTicketResponse {
    Cdr(VerifyTicketStatus),
    Error(ErrorResponse),
}

pub struct VerifyTicketStatus {
    pub cdr_base64: String,
    pub status_code: String,
}

pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ClientSUNAT {
    pub async fn send_file(
        &self,
        target: &SendFileTarget,
        file: &File,
        credentials: &Credentials,
    ) -> Result<SendFileResponse, ErrorClientSUNAT> {
        match &target {
            SendFileTarget::Soap(url, action) => {
                let envelope_body = match action {
                    SoapFileTargetAction::Bill | SoapFileTargetAction::Pack => {
                        BodyData::SendBill(FileData {
                            filename: file.name.clone(),
                            file_content: file.base_64.clone(),
                        })
                    }
                    SoapFileTargetAction::Summary => BodyData::SendSummary(FileData {
                        filename: file.name.clone(),
                        file_content: file.base_64.clone(),
                    }),
                };

                let envelope = EnvelopeData {
                    username: credentials.username.clone(),
                    password: credentials.password.clone(),
                    body: envelope_body,
                };

                let soap_action = match action {
                    SoapFileTargetAction::Bill => "urn:sendBill",
                    SoapFileTargetAction::Summary => "urn:sendSummary",
                    SoapFileTargetAction::Pack => "urn:sendPack",
                };

                let body = envelope.to_string_xml()?;

                let response = HTTP_CLIENT
                    .clone()
                    .post(url)
                    .body(body)
                    .header("Content-Type", "text/xml; charset=utf-8")
                    .header("SOAPAction", soap_action)
                    .send()
                    .await?;

                let response_status = response.status();
                if !response_status.is_success() {
                    return Err(ErrorClientSUNAT {
                        kind: Layer::ServiceUnavailable(response_status.to_string()),
                    });
                }

                let body = response.text().await?;
                let response_body = SendFileXmlResponse::from_str(&body)?;

                Ok(response_body.into())
            }
            SendFileTarget::Rest(_, _) => Err(ErrorClientSUNAT {
                kind: Layer::ServiceUnavailable(
                    "SendFileTarget::Rest not implemented yet".to_string(),
                ),
            }),
        }
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
        credentials: &Credentials,
    ) -> Result<VerifyTicketResponse, ErrorClientSUNAT> {
        match &target {
            VerifyTicketTarget::Soap(url) => {
                let envelope = EnvelopeData {
                    username: credentials.username.clone(),
                    password: credentials.password.clone(),
                    body: BodyData::VerifyTicket(ticket.to_string()),
                };

                let body = envelope.to_string_xml()?;

                let response = HTTP_CLIENT
                    .clone()
                    .post(url)
                    .body(body)
                    .header("Content-Type", "text/xml; charset=utf-8")
                    .header("SOAPAction", "urn:getStatus")
                    .send()
                    .await?;

                let response_status = response.status();
                if !response_status.is_success() {
                    return Err(ErrorClientSUNAT {
                        kind: Layer::ServiceUnavailable(response_status.to_string()),
                    });
                }

                let body = response.text().await?;
                let response_body = VerifyTicketXmlResponse::from_str(&body)?;

                Ok(response_body.into())
            }
            VerifyTicketTarget::Rest(_) => Err(ErrorClientSUNAT {
                kind: Layer::ServiceUnavailable(
                    "SendFileTarget::Rest not implemented yet".to_string(),
                ),
            }),
        }
    }
}

impl From<SendFileXmlResponse> for SendFileResponse {
    fn from(value: SendFileXmlResponse) -> Self {
        match value {
            SendFileXmlResponse::Cdr(cdr) => Self::Cdr(cdr),
            SendFileXmlResponse::Ticket(ticket) => Self::Ticket(ticket),
            SendFileXmlResponse::Fault(error) => Self::Error(ErrorResponse {
                code: error.code,
                message: error.message,
            }),
        }
    }
}

impl From<VerifyTicketXmlResponse> for VerifyTicketResponse {
    fn from(value: VerifyTicketXmlResponse) -> Self {
        match value {
            VerifyTicketXmlResponse::Status(status) => Self::Cdr(VerifyTicketStatus {
                cdr_base64: status.cdr_base64,
                status_code: status.status_code,
            }),
            VerifyTicketXmlResponse::Fault(error) => Self::Error(ErrorResponse {
                code: error.code,
                message: error.message,
            }),
        }
    }
}

impl From<reqwest::Error> for ErrorClientSUNAT {
    fn from(error: reqwest::Error) -> Self {
        Self {
            kind: Layer::Request(error),
        }
    }
}

impl From<tera::Error> for ErrorClientSUNAT {
    fn from(value: tera::Error) -> Self {
        Self {
            kind: Layer::Templating(value.to_string()),
        }
    }
}

impl From<SendFileXmlResponseFromStrError> for ErrorClientSUNAT {
    fn from(_: SendFileXmlResponseFromStrError) -> Self {
        Self {
            kind: Layer::ReadingResponse,
        }
    }
}

impl From<VerifyTicketXmlResponseFromStrError> for ErrorClientSUNAT {
    fn from(_: VerifyTicketXmlResponseFromStrError) -> Self {
        Self {
            kind: Layer::ReadingResponse,
        }
    }
}
