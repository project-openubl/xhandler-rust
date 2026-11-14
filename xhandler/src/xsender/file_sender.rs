use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::xsender::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::xsender::exchange::{
    File, SendFileResponse, SunatClientError, VerifyTicketResponse, VerifyTicketStatus,
};
use crate::xsender::fault::SunatFault;
use crate::xsender::models::{
    Credentials, DocumentType, SendFileTarget, SoapFileTargetAction, Urls, VerifyTicketTarget,
};
use crate::xsender::soap::cdr::CdrMetadata;
#[cfg(feature = "http")]
use crate::xsender::sunat_client::SunatClient;
use crate::xsender::ubl_file::{UblFile, UblMetadata};
use crate::xsender::zip_archive::{create_zip, read_cdr_from_base64_zip};

pub struct FileSender {
    pub urls: Urls,
    pub credentials: Credentials,
}

#[derive(Debug, thiserror::Error)]
pub enum FileSenderError {
    #[error("Couldn't infer the target destinations of file")]
    TargetDiscovery,
    #[error("No URL is set for guias de remision (SUNAT has no beta for them)")]
    NoDespatchUrl,
    #[error("An error while creating/reading the zip file")]
    Zip(ZipError),
    #[error(transparent)]
    SunatClient(#[from] SunatClientError),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<ZipError> for FileSenderError {
    fn from(e: ZipError) -> Self {
        Self::Zip(e)
    }
}

pub struct SendFileResponseWrapper {
    pub response: SendFileAggregatedResponse,
    pub send_file_target: SendFileTarget,
    pub verify_ticket_target: Option<VerifyTicketTarget>,
}

pub enum SendFileAggregatedResponse {
    /// (CDR in base64, CDR base64 decoded into an object)
    Cdr(String, CdrMetadata),
    Ticket(String),
    Fault(SunatFault),
}

pub struct VerifyTicketResponseWrapper {
    pub response: VerifyTicketAggregatedResponse,
}

pub enum VerifyTicketAggregatedResponse {
    Cdr(VerifyTicketStatus, CdrMetadata),
    /// SUNAT answered without a CDR, e.g. status code 98 (still processing).
    /// Use [`crate::xsender::models::Status::from_code_str`] to classify `status_code`.
    NoCdr {
        status_code: String,
    },
    Fault(SunatFault),
}

/// What [`FileSender::send_file`] and [`FileSender::send_pack`] send: the zip
/// and the services for it and its ticket
pub struct PreparedSend {
    pub file: File,
    pub send_file_target: SendFileTarget,
    pub verify_ticket_target: Option<VerifyTicketTarget>,
}

/// The zip of a document (named as SUNAT expects) and its services
pub fn prepare_send_file(xml: &UblFile, urls: &Urls) -> Result<PreparedSend, FileSenderError> {
    let metadata = checked_metadata(xml, urls)?;
    let send_file_target = send_file_target(
        &metadata.document_type,
        &metadata.voided_line_document_type_code,
        urls,
    )
    .ok_or(FileSenderError::TargetDiscovery)?;
    let verify_ticket_target = verify_ticket_target(
        &metadata.document_type,
        &metadata.voided_line_document_type_code,
        urls,
    );
    let name = sunat_filename(&metadata)?;
    let entries = [(format!("{name}.xml"), xml.bytes())];
    Ok(PreparedSend {
        file: zip_file(format!("{name}.zip"), &entries)?,
        send_file_target,
        verify_ticket_target,
    })
}

/// The zip of several invoices sent together (sendPack)
pub fn prepare_send_pack(
    pack_name: &str,
    files: &[UblFile],
    urls: &Urls,
) -> Result<PreparedSend, FileSenderError> {
    let entries = files
        .iter()
        .map(|xml| {
            Ok((
                format!("{}.xml", sunat_filename(&xml.metadata()?)?),
                xml.bytes(),
            ))
        })
        .collect::<Result<Vec<_>, FileSenderError>>()?;
    Ok(PreparedSend {
        file: zip_file(format!("{pack_name}.zip"), &entries)?,
        send_file_target: SendFileTarget::Soap(urls.invoice.clone(), SoapFileTargetAction::Pack),
        verify_ticket_target: Some(VerifyTicketTarget::Soap(urls.invoice.clone())),
    })
}

/// The service that answers the tickets of a document
pub fn file_verify_ticket_target(
    xml: &UblFile,
    urls: &Urls,
) -> Result<VerifyTicketTarget, FileSenderError> {
    let metadata = checked_metadata(xml, urls)?;
    verify_ticket_target(
        &metadata.document_type,
        &metadata.voided_line_document_type_code,
        urls,
    )
    .ok_or(FileSenderError::TargetDiscovery)
}

/// Reads the CDR in the answer
impl TryFrom<SendFileResponse> for SendFileAggregatedResponse {
    type Error = FileSenderError;

    fn try_from(response: SendFileResponse) -> Result<Self, Self::Error> {
        Ok(match response {
            SendFileResponse::Cdr(cdr_base64) => {
                let cdr_metadata = read_cdr_metadata(&cdr_base64)?;
                Self::Cdr(cdr_base64, cdr_metadata)
            }
            SendFileResponse::Ticket(ticket) => Self::Ticket(ticket),
            SendFileResponse::Fault(fault) => Self::Fault(fault),
        })
    }
}

/// Reads the CDR in the answer
impl TryFrom<VerifyTicketResponse> for VerifyTicketAggregatedResponse {
    type Error = FileSenderError;

    fn try_from(response: VerifyTicketResponse) -> Result<Self, Self::Error> {
        Ok(match response {
            VerifyTicketResponse::Cdr(status) => {
                let cdr_metadata = read_cdr_metadata(&status.cdr_base64)?;
                Self::Cdr(status, cdr_metadata)
            }
            VerifyTicketResponse::NoCdr { status_code } => Self::NoCdr { status_code },
            VerifyTicketResponse::Fault(fault) => Self::Fault(fault),
        })
    }
}

#[cfg(feature = "http")]
impl FileSender {
    pub async fn send_file(
        &self,
        xml: &UblFile,
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        self.send(prepare_send_file(xml, &self.urls)?).await
    }

    pub async fn send_pack(
        &self,
        pack_name: &str,
        files: &[UblFile],
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        self.send(prepare_send_pack(pack_name, files, &self.urls)?)
            .await
    }

    async fn send(
        &self,
        prepared: PreparedSend,
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        let response = SunatClient::default()
            .send_file(
                &prepared.send_file_target,
                &prepared.file,
                &self.credentials,
            )
            .await?;
        Ok(SendFileResponseWrapper {
            response: response.try_into()?,
            send_file_target: prepared.send_file_target,
            verify_ticket_target: prepared.verify_ticket_target,
        })
    }

    pub async fn verify_file_ticket(
        &self,
        xml: &UblFile,
        ticket: &str,
    ) -> Result<VerifyTicketResponseWrapper, FileSenderError> {
        let target = file_verify_ticket_target(xml, &self.urls)?;
        self.verify_ticket(&target, ticket).await
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
    ) -> Result<VerifyTicketResponseWrapper, FileSenderError> {
        let response = SunatClient::default()
            .verify_ticket(target, ticket, &self.credentials)
            .await?;
        Ok(VerifyTicketResponseWrapper {
            response: response.try_into()?,
        })
    }
}

fn checked_metadata(xml: &UblFile, urls: &Urls) -> Result<UblMetadata, FileSenderError> {
    let metadata = xml.metadata()?;
    if metadata.document_type == DocumentType::DESPATCH_ADVICE && urls.despatch.is_empty() {
        return Err(FileSenderError::NoDespatchUrl);
    }
    Ok(metadata)
}

fn zip_file(filename: String, entries: &[(String, &[u8])]) -> Result<File, FileSenderError> {
    Ok(File {
        filename,
        zip_base64: general_purpose::STANDARD.encode(create_zip(entries)?),
    })
}

fn read_cdr_metadata(cdr_base64: &str) -> Result<CdrMetadata, FileSenderError> {
    Ok(CdrMetadata::from_str(&read_cdr_from_base64_zip(
        cdr_base64,
    )?)?)
}

fn sunat_filename(metadata: &UblMetadata) -> Result<String, FileSenderError> {
    filename_formatted_without_extension(
        &metadata.document_type,
        &metadata.document_id,
        &metadata.ruc,
    )
    .map_err(|_| FileSenderError::TargetDiscovery)
}
