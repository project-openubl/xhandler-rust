//! The answers `parse` gives back. They are the contract with the bindings:
//! examples/schema.rs describes them and the Go, Python and Java types are
//! generated from that schema (docs/adrs/BINDINGS.md), so a field added here
//! reaches every binding with the next release.

use serde::Serialize;
use xhandler::xsender::exchange::ConsultStatus as SunatConsultStatus;
use xhandler::xsender::prelude::{CdrMetadata, Status as SunatStatus, SunatFault};

/// The SUNAT verdict of a document, from its response code
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(extend("x-enum-varnames" = ["Accepted", "Rejected", "Exception", "InProgress", "Unknown"]))
)]
pub enum Status {
    #[serde(rename = "ACEPTADO")]
    Accepted,
    #[serde(rename = "RECHAZADO")]
    Rejected,
    #[serde(rename = "EXCEPCION")]
    Exception,
    #[serde(rename = "EN_PROCESO")]
    InProgress,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl From<SunatStatus> for Status {
    fn from(status: SunatStatus) -> Self {
        match status {
            SunatStatus::Accepted => Self::Accepted,
            SunatStatus::Rejected => Self::Rejected,
            SunatStatus::Exception => Self::Exception,
            SunatStatus::InProgress => Self::InProgress,
            SunatStatus::Unknown => Self::Unknown,
        }
    }
}

/// The receipt SUNAT gives back (Constancia de Recepción)
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "CDR"))]
pub struct Cdr {
    /// The CDR zip
    #[cfg_attr(feature = "schema", schemars(extend("format" = "byte")))]
    pub zip: String,
    pub response_code: String,
    pub description: String,
    pub notes: Vec<String>,
    /// For an accepted guía de remisión, the URL its printed copy's QR code
    /// must encode; null for other documents
    pub document_description: Option<String>,
    pub status: Status,
    /// The ticket status, only when verifying a ticket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

impl Cdr {
    pub fn new(zip: String, cdr: CdrMetadata) -> Self {
        Self {
            zip,
            status: cdr.status().into(),
            response_code: cdr.response_code,
            description: cdr.description,
            notes: cdr.notes,
            document_description: cdr.document_description,
            status_code: None,
        }
    }
}

/// An error answered by SUNAT
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Fault {
    /// SUNAT's code as given: the SOAP faultcode (e.g. "soap-env:Client.0111")
    /// or the error code of a REST answer (e.g. "1033")
    pub code: String,
    /// The number in `code` (111), when it has one
    pub sunat_code: Option<u32>,
    /// The status of the answer, e.g. 500 for SOAP faults or 422 for REST
    /// errors
    pub http_status: Option<u16>,
    pub message: String,
    pub status: Status,
}

impl From<SunatFault> for Fault {
    fn from(fault: SunatFault) -> Self {
        Self {
            sunat_code: fault.sunat_code(),
            status: fault.status().into(),
            code: fault.code,
            http_status: fault.http_status,
            message: fault.message,
        }
    }
}

/// A ticket status without CDR, e.g. "98" (in progress)
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "NoCDR"))]
pub struct NoCdr {
    pub status_code: String,
    pub status: Status,
}

/// A status of the consult services: e.g. "0001" (accepted) for getStatus, or
/// "0" for validaCDPcriterios
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ConsultStatus {
    pub status_code: String,
    pub status_message: Option<String>,
    /// The CDR zip of getStatusCdr
    #[cfg_attr(feature = "schema", schemars(extend("format" = "byte")))]
    pub cdr_zip: Option<String>,
}

impl From<SunatConsultStatus> for ConsultStatus {
    fn from(status: SunatConsultStatus) -> Self {
        Self {
            status_code: status.status_code,
            status_message: status.status_message,
            cdr_zip: status.cdr_base64,
        }
    }
}

/// The answer to a sent file: a CDR, a ticket to verify later (summaries,
/// voided documents, packs, guías) or a fault
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum SendFileAnswer {
    Cdr(Cdr),
    Ticket(String),
    Fault(Fault),
}

/// The answer to a ticket: a CDR, no CDR yet (e.g. in progress) or a fault
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum VerifyResult {
    Cdr(Cdr),
    NoCdr(NoCdr),
    Fault(Fault),
}

/// The answer of the consult and validation services
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ConsultResult {
    Status(ConsultStatus),
    Fault(Fault),
}

/// A token of the guías' REST API. The bindings use it internally, so it
/// isn't part of the schema.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenAnswer {
    Token {
        access_token: String,
        expires_in: u64,
    },
}
