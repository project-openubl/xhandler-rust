//! The SUNAT calls of xsender without IO: the bindings send the HTTP requests
//! built here with their own clients and give the answers back to parse.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use xsender::exchange::{Answer, Call, File, HttpRequest, SunatClientError};
use xsender::prelude::*;
// These xsender functions share their names with the ops
use xsender::prelude::{
    prepare_send_file as prepare_send_file_of, prepare_send_pack as prepare_send_pack_of,
};

use crate::bytes::Base64;
use crate::ops::{parse_input, Error, ErrorKind, OpResult};

/// `{xml, urls}` -> the zip of the document and its services
pub fn prepare_send_file(input: Value) -> OpResult {
    #[derive(Deserialize)]
    struct Input {
        xml: Base64,
        urls: Urls,
    }
    let Input { xml, urls } = parse_input(input)?;
    let prepared = prepare_send_file_of(&ubl_file(xml)?, &urls).map_err(invalid_input)?;
    Ok(prepared_json(prepared))
}

/// `{name, files: [xml], urls}` -> the zip of the documents (sendPack)
pub fn prepare_send_pack(input: Value) -> OpResult {
    #[derive(Deserialize)]
    struct Input {
        name: String,
        files: Vec<Base64>,
        urls: Urls,
    }
    let Input { name, files, urls } = parse_input(input)?;
    let files = files
        .into_iter()
        .map(ubl_file)
        .collect::<Result<Vec<_>, _>>()?;
    let prepared = prepare_send_pack_of(&name, &files, &urls).map_err(invalid_input)?;
    Ok(prepared_json(prepared))
}

/// `{xml, urls}` -> the service that answers the tickets of the document
pub fn verify_ticket_target(input: Value) -> OpResult {
    #[derive(Deserialize)]
    struct Input {
        xml: Base64,
        urls: Urls,
    }
    let Input { xml, urls } = parse_input(input)?;
    let target = file_verify_ticket_target(&ubl_file(xml)?, &urls).map_err(invalid_input)?;
    Ok(TargetJson::from(&target).into())
}

/// `{call, credentials, token?}` -> the HTTP request of the call
pub fn request(input: Value) -> OpResult {
    #[derive(Deserialize)]
    struct Input {
        call: CallJson,
        credentials: Credentials,
        token: Option<String>,
    }
    let Input {
        call,
        credentials,
        token,
    } = parse_input(input)?;
    let request: HttpRequest = Call::try_from(call)?
        .request(&credentials, token.as_deref())
        .map_err(invalid_input)?;
    Ok(serde_json::to_value(request).expect("requests are serializable"))
}

/// `{call, status, body}` -> the answer, with the CDR read and the SUNAT status
pub fn parse(input: Value) -> OpResult {
    #[derive(Deserialize)]
    struct Input {
        call: CallJson,
        status: u16,
        body: String,
    }
    let Input { call, status, body } = parse_input(input)?;
    let answer = Call::try_from(call)?
        .parse(status, &body)
        .map_err(response_error)?;
    answer_json(answer)
}

fn ubl_file(xml: Base64) -> Result<UblFile, Error> {
    UblFile::from_bytes(xml.0).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

fn invalid_input(error: impl Into<anyhow::Error>) -> Error {
    Error::new(ErrorKind::InvalidInput, error)
}

fn response_error(error: SunatClientError) -> Error {
    let kind = match error {
        SunatClientError::RateLimited => ErrorKind::RateLimited,
        _ => ErrorKind::Response,
    };
    Error::new(kind, error)
}

fn prepared_json(prepared: PreparedSend) -> Value {
    json!({
        "file": FileJson::from(prepared.file),
        "send_target": TargetJson::from(&prepared.send_file_target),
        "verify_ticket_target": prepared.verify_ticket_target.as_ref().map(TargetJson::from),
    })
}

/// A zip file: `{filename, zip}` (base64)
#[derive(Deserialize, Serialize)]
struct FileJson {
    filename: String,
    zip: String,
}

impl From<File> for FileJson {
    fn from(file: File) -> Self {
        Self {
            filename: file.filename,
            zip: file.zip_base64,
        }
    }
}

impl From<FileJson> for File {
    fn from(file: FileJson) -> Self {
        Self {
            filename: file.filename,
            zip_base64: file.zip,
        }
    }
}

/// Where a file or a ticket goes: `{protocol: soap|rest, url, action?}`,
/// action being bill, summary or pack for SOAP files
#[derive(Deserialize, Serialize)]
struct TargetJson {
    protocol: Protocol,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Protocol {
    Soap,
    Rest,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    Bill,
    Summary,
    Pack,
}

impl From<&SendFileTarget> for TargetJson {
    fn from(target: &SendFileTarget) -> Self {
        match target {
            SendFileTarget::Soap(url, action) => Self {
                protocol: Protocol::Soap,
                url: url.clone(),
                action: Some(match action {
                    SoapFileTargetAction::Bill => Action::Bill,
                    SoapFileTargetAction::Summary => Action::Summary,
                    SoapFileTargetAction::Pack => Action::Pack,
                }),
            },
            SendFileTarget::Rest(url, _) => Self {
                protocol: Protocol::Rest,
                url: url.clone(),
                action: None,
            },
        }
    }
}

impl From<&VerifyTicketTarget> for TargetJson {
    fn from(target: &VerifyTicketTarget) -> Self {
        let (protocol, url) = match target {
            VerifyTicketTarget::Soap(url) => (Protocol::Soap, url),
            VerifyTicketTarget::Rest(url) => (Protocol::Rest, url),
        };
        Self {
            protocol,
            url: url.clone(),
            action: None,
        }
    }
}

impl From<TargetJson> for Value {
    fn from(target: TargetJson) -> Self {
        serde_json::to_value(target).expect("targets are serializable")
    }
}

impl TargetJson {
    fn send_file_target(self) -> Result<SendFileTarget, Error> {
        Ok(match (self.protocol, self.action) {
            (Protocol::Rest, _) => {
                SendFileTarget::Rest(self.url, RestFileTargetAction::SendDocument)
            }
            (Protocol::Soap, Some(action)) => SendFileTarget::Soap(
                self.url,
                match action {
                    Action::Bill => SoapFileTargetAction::Bill,
                    Action::Summary => SoapFileTargetAction::Summary,
                    Action::Pack => SoapFileTargetAction::Pack,
                },
            ),
            (Protocol::Soap, None) => {
                return Err(invalid_input(anyhow::anyhow!(
                    "a SOAP file target needs an action"
                )))
            }
        })
    }

    fn verify_ticket_target(self) -> VerifyTicketTarget {
        match self.protocol {
            Protocol::Soap => VerifyTicketTarget::Soap(self.url),
            Protocol::Rest => VerifyTicketTarget::Rest(self.url),
        }
    }
}

/// A SUNAT call, e.g. `{"send_file": {"target": ..., "file": ...}}` or
/// `{"token": {"url"?}}`
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CallJson {
    SendFile {
        target: TargetJson,
        file: FileJson,
    },
    VerifyTicket {
        target: TargetJson,
        ticket: String,
    },
    GetStatus {
        url: String,
        document: ConsultDocument,
    },
    GetStatusCdr {
        url: String,
        document: ConsultDocument,
    },
    ValidateCdp {
        url: String,
        cdp: ValidateCdp,
    },
    ValidateFile {
        url: String,
        file: FileJson,
    },
    Token {
        url: Option<String>,
    },
}

impl TryFrom<CallJson> for Call {
    type Error = Error;

    fn try_from(call: CallJson) -> Result<Self, Error> {
        Ok(match call {
            CallJson::SendFile { target, file } => Call::SendFile {
                target: target.send_file_target()?,
                file: file.into(),
            },
            CallJson::VerifyTicket { target, ticket } => Call::VerifyTicket {
                target: target.verify_ticket_target(),
                ticket,
            },
            CallJson::GetStatus { url, document } => Call::GetStatus { url, document },
            CallJson::GetStatusCdr { url, document } => Call::GetStatusCdr { url, document },
            CallJson::ValidateCdp { url, cdp } => Call::ValidateCdp { url, cdp },
            CallJson::ValidateFile { url, file } => Call::ValidateFile {
                url,
                file: file.into(),
            },
            CallJson::Token { url } => Call::Token { url },
        })
    }
}

fn answer_json(answer: Answer) -> OpResult {
    Ok(match answer {
        Answer::SendFile(response) => {
            match SendFileAggregatedResponse::try_from(response).map_err(cdr_error)? {
                SendFileAggregatedResponse::Cdr(zip, cdr) => json!({ "cdr": cdr_json(&zip, &cdr) }),
                SendFileAggregatedResponse::Ticket(ticket) => json!({ "ticket": ticket }),
                SendFileAggregatedResponse::Fault(fault) => json!({ "fault": fault_json(&fault) }),
            }
        }
        Answer::VerifyTicket(response) => {
            match VerifyTicketAggregatedResponse::try_from(response).map_err(cdr_error)? {
                VerifyTicketAggregatedResponse::Cdr(status, cdr) => {
                    let mut cdr_value = cdr_json(&status.cdr_base64, &cdr);
                    cdr_value["status_code"] = status.status_code.into();
                    json!({ "cdr": cdr_value })
                }
                VerifyTicketAggregatedResponse::NoCdr { status_code } => json!({ "no_cdr": {
                    "status_code": status_code,
                    "status": Status::from_code_str(&status_code).to_string(),
                }}),
                VerifyTicketAggregatedResponse::Fault(fault) => {
                    json!({ "fault": fault_json(&fault) })
                }
            }
        }
        Answer::Consult(ConsultResponse::Status(status)) => json!({ "status": {
            "status_code": status.status_code,
            "status_message": status.status_message,
            "cdr_zip": status.cdr_base64,
        }}),
        Answer::Consult(ConsultResponse::Fault(fault)) => json!({ "fault": fault_json(&fault) }),
        Answer::Token(token) => json!({ "token": {
            "access_token": token.access_token,
            "expires_in": token.expires_in,
        }}),
    })
}

/// The CDR zip of the answer can't be read
fn cdr_error(error: FileSenderError) -> Error {
    Error::new(ErrorKind::Response, error)
}

fn cdr_json(zip: &str, cdr: &CdrMetadata) -> Value {
    json!({
        "zip": zip,
        "response_code": cdr.response_code,
        "description": cdr.description,
        "notes": cdr.notes,
        "status": cdr.status().to_string(),
    })
}

fn fault_json(fault: &SunatFault) -> Value {
    json!({
        "code": fault.code,
        "message": fault.message,
        "status": fault.status().to_string(),
    })
}
