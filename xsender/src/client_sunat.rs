use std::str::FromStr;

use crate::models::{
    Credentials, RestFileTargetAction, SendFileTarget, SoapFileTargetAction, VerifyTicketTarget,
};
use crate::soap::cdr::{Cdr, CdrReadError};
use crate::soap::envelope::{BodyData, EnvelopeData, FileData, ToStringXml};
use crate::soap::send_file_response::{SendFileXmlResponse, SendFileXmlResponseFromStrError};
use crate::soap::verify_ticket_response::{
    VerifyTicketXmlResponse, VerifyTicketXmlResponseFromStrError,
};
use crate::zip_manager::{extract_cdr_from_base64_zip, ExtractCdrFromBase64ZipError};

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
}

pub enum SendFileResponse {
    Ok(String, Cdr),
    Ticket(String),
    Error(String, String),
}

pub enum VerifyTicketResponse {
    Ok(String, Cdr, String),
    Error(String, String),
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

                let body = reqwest::Client::new()
                    .post(url)
                    .body(body)
                    .header("Content-Type", "text/xml; charset=utf-8")
                    .header("SOAPAction", soap_action)
                    .send()
                    .await?
                    .text()
                    .await?;

                let response_body = SendFileXmlResponse::from_str(&body)?;

                let result = match response_body {
                    SendFileXmlResponse::Cdr(cdr_base64) => {
                        let cdr_xml = extract_cdr_from_base64_zip(&cdr_base64)?;
                        let cdr = Cdr::from_str(&cdr_xml)?;
                        SendFileResponse::Ok(cdr_base64, cdr)
                    }
                    SendFileXmlResponse::Ticket(ticket) => SendFileResponse::Ticket(ticket),
                    SendFileXmlResponse::Fault(code, description) => {
                        SendFileResponse::Error(code, description)
                    }
                };

                Ok(result)
            }
            SendFileTarget::Rest(url, action) => {
                let path = match action {
                    RestFileTargetAction::SendDocument => "/comprobantes",
                };
                let _ = reqwest::Client::new()
                    .post(format!("{url}/{path}/{}", file.name))
                    .body("test")
                    .header("Content-Type", "application/json")
                    .send()
                    .await?
                    .text()
                    .await?;
                Ok(SendFileResponse::Ticket("".to_string()))
            }
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

                let body = reqwest::Client::new()
                    .post(url)
                    .body(body)
                    .header("Content-Type", "text/xml; charset=utf-8")
                    .header("SOAPAction", "urn:getStatus")
                    .send()
                    .await?
                    .text()
                    .await?;

                let response_body = VerifyTicketXmlResponse::from_str(&body)?;

                match response_body {
                    VerifyTicketXmlResponse::Cdr(cdr_base64, status_code) => {
                        let cdr_xml = extract_cdr_from_base64_zip(&cdr_base64)?;
                        let cdr = Cdr::from_str(&cdr_xml)?;
                        let result = VerifyTicketResponse::Ok(cdr_base64, cdr, status_code);

                        Ok(result)
                    }
                    VerifyTicketXmlResponse::Fault(code, description) => {
                        let result = VerifyTicketResponse::Error(code, description);

                        Ok(result)
                    }
                }
            }
            VerifyTicketTarget::Rest(url) => {
                let _ = reqwest::Client::new()
                    .post(format!("{url}/ticket"))
                    .body("test")
                    .header("Content-Type", "application/json")
                    .send()
                    .await?
                    .text()
                    .await?;
                Ok(VerifyTicketResponse::Error("".to_string(), "".to_string()))
            }
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

impl From<ExtractCdrFromBase64ZipError> for ErrorClientSUNAT {
    fn from(_: ExtractCdrFromBase64ZipError) -> Self {
        Self {
            kind: Layer::ReadingResponse,
        }
    }
}

impl From<CdrReadError> for ErrorClientSUNAT {
    fn from(_: CdrReadError) -> Self {
        Self {
            kind: Layer::ReadingResponse,
        }
    }
}
