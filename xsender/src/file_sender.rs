use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::fault::SunatFault;
use crate::models::{
    Credentials, DocumentType, SendFileTarget, SoapFileTargetAction, Urls, VerifyTicketTarget,
};
use crate::prelude::VerifyTicketStatus;
use crate::soap::cdr::CdrMetadata;
use crate::sunat_client::{
    File, SendFileResponse, SunatClient, SunatClientError, VerifyTicketResponse,
};
use crate::ubl_file::{UblFile, UblMetadata};
use crate::zip_archive::{create_zip, read_cdr_from_base64_zip};

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
    /// Use [`crate::models::Status::from_code_str`] to classify `status_code`.
    NoCdr {
        status_code: String,
    },
    Fault(SunatFault),
}

impl FileSender {
    pub async fn send_file(
        &self,
        xml: &UblFile,
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        let metadata = self.checked_metadata(xml)?;
        let send_target = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        )
        .ok_or(FileSenderError::TargetDiscovery)?;
        let verify_ticket_target = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        );

        let name = sunat_filename(&metadata)?;
        let entries = [(format!("{name}.xml"), xml.bytes())];
        self.send(
            format!("{name}.zip"),
            &entries,
            send_target,
            verify_ticket_target,
        )
        .await
    }

    /// Sends several documents in one zip with sendPack (SUNAT answers with a
    /// ticket). `pack_name` is the zip name without extension, e.g.
    /// "20123456789-LT-20240131-1". SUNAT only accepts packs in production.
    pub async fn send_pack(
        &self,
        pack_name: &str,
        files: &[UblFile],
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        let entries = files
            .iter()
            .map(|xml| {
                Ok((
                    format!("{}.xml", sunat_filename(&xml.metadata()?)?),
                    xml.bytes(),
                ))
            })
            .collect::<Result<Vec<_>, FileSenderError>>()?;
        let send_target =
            SendFileTarget::Soap(self.urls.invoice.clone(), SoapFileTargetAction::Pack);
        let verify_ticket_target = Some(VerifyTicketTarget::Soap(self.urls.invoice.clone()));
        self.send(
            format!("{pack_name}.zip"),
            &entries,
            send_target,
            verify_ticket_target,
        )
        .await
    }

    async fn send(
        &self,
        zip_name: String,
        entries: &[(String, &[u8])],
        send_target: SendFileTarget,
        verify_ticket_target: Option<VerifyTicketTarget>,
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        let file = File {
            filename: zip_name,
            zip_base64: general_purpose::STANDARD.encode(create_zip(entries)?),
        };

        let result = SunatClient::default()
            .send_file(&send_target, &file, &self.credentials)
            .await?;

        let response = match result {
            SendFileResponse::Cdr(cdr_base64) => {
                let cdr_xml = read_cdr_from_base64_zip(&cdr_base64)?;
                let cdr_metadata = CdrMetadata::from_str(&cdr_xml)?;
                SendFileAggregatedResponse::Cdr(cdr_base64, cdr_metadata)
            }
            SendFileResponse::Ticket(ticket) => SendFileAggregatedResponse::Ticket(ticket),
            SendFileResponse::Fault(fault) => SendFileAggregatedResponse::Fault(fault),
        };

        Ok(SendFileResponseWrapper {
            response,
            send_file_target: send_target,
            verify_ticket_target,
        })
    }

    /// Checks the ticket SUNAT answered when `xml` was sent; the service to
    /// ask is inferred from the document, like in [`Self::send_file`].
    pub async fn verify_file_ticket(
        &self,
        xml: &UblFile,
        ticket: &str,
    ) -> Result<VerifyTicketResponseWrapper, FileSenderError> {
        let metadata = self.checked_metadata(xml)?;
        let target = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        )
        .ok_or(FileSenderError::TargetDiscovery)?;
        self.verify_ticket(&target, ticket).await
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
    ) -> Result<VerifyTicketResponseWrapper, FileSenderError> {
        let client = SunatClient::default();
        let result = client
            .verify_ticket(target, ticket, &self.credentials)
            .await?;

        let response = match result {
            VerifyTicketResponse::Cdr(status) => {
                let cdr_xml = read_cdr_from_base64_zip(&status.cdr_base64)?;
                let cdr_metadata = CdrMetadata::from_str(&cdr_xml)?;
                VerifyTicketAggregatedResponse::Cdr(status, cdr_metadata)
            }
            VerifyTicketResponse::NoCdr { status_code } => {
                VerifyTicketAggregatedResponse::NoCdr { status_code }
            }
            VerifyTicketResponse::Fault(fault) => VerifyTicketAggregatedResponse::Fault(fault),
        };

        Ok(VerifyTicketResponseWrapper { response })
    }
}

/// File name SUNAT expects, without extension, e.g. "12345678912-01-F001-1"
impl FileSender {
    /// Metadata of a document this sender has a URL for
    fn checked_metadata(&self, xml: &UblFile) -> Result<UblMetadata, FileSenderError> {
        let metadata = xml.metadata()?;
        if metadata.document_type == DocumentType::DESPATCH_ADVICE && self.urls.despatch.is_empty()
        {
            return Err(FileSenderError::NoDespatchUrl);
        }
        Ok(metadata)
    }
}

fn sunat_filename(metadata: &UblMetadata) -> Result<String, FileSenderError> {
    filename_formatted_without_extension(
        &metadata.document_type,
        &metadata.document_id,
        &metadata.ruc,
    )
    .map_err(|_| FileSenderError::TargetDiscovery)
}
