use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::models::{Credentials, SendFileTarget, SoapFileTargetAction, Urls, VerifyTicketTarget};
use crate::prelude::VerifyTicketStatus;
use crate::soap::cdr::CdrMetadata;
use crate::sunat_client::{
    ClientSunatErr, ErrorResponse, File, SendFileResponse, SunatClient, VerifyTicketResponse,
};
use crate::ubl_file::{UblFile, UblMetadata};
use crate::zip_manager::{create_zip, read_cdr_from_base64_zip};

pub struct FileSender {
    pub urls: Urls,
    pub credentials: Credentials,
}

#[derive(Debug, thiserror::Error)]
pub enum FileSenderErr {
    #[error("Couldn't infer the target destinations of file")]
    TargetDiscovery,
    #[error("An error while creating/reading the zip file")]
    ZipRead(ZipError),
    #[error(transparent)]
    ClientSunat(#[from] ClientSunatErr),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<ZipError> for FileSenderErr {
    fn from(e: ZipError) -> Self {
        Self::ZipRead(e)
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
    Error(ErrorResponse),
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
    Error(ErrorResponse),
}

impl FileSender {
    pub async fn send_file(&self, xml: &UblFile) -> Result<SendFileResponseWrapper, FileSenderErr> {
        let metadata = xml.metadata()?;
        let send_target = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        )
        .ok_or(FileSenderErr::TargetDiscovery)?;
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
    ) -> Result<SendFileResponseWrapper, FileSenderErr> {
        let entries = files
            .iter()
            .map(|xml| {
                Ok((
                    format!("{}.xml", sunat_filename(&xml.metadata()?)?),
                    xml.bytes(),
                ))
            })
            .collect::<Result<Vec<_>, FileSenderErr>>()?;
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
    ) -> Result<SendFileResponseWrapper, FileSenderErr> {
        let file = File {
            name: zip_name,
            base_64: general_purpose::STANDARD.encode(create_zip(entries)?),
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
            SendFileResponse::Error(error) => SendFileAggregatedResponse::Error(error),
        };

        Ok(SendFileResponseWrapper {
            response,
            send_file_target: send_target,
            verify_ticket_target,
        })
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
    ) -> Result<VerifyTicketResponseWrapper, FileSenderErr> {
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
            VerifyTicketResponse::Error(error) => VerifyTicketAggregatedResponse::Error(error),
        };

        Ok(VerifyTicketResponseWrapper { response })
    }
}

/// File name SUNAT expects, without extension, e.g. "12345678912-01-F001-1"
fn sunat_filename(metadata: &UblMetadata) -> Result<String, FileSenderErr> {
    filename_formatted_without_extension(
        &metadata.document_type,
        &metadata.document_id,
        &metadata.ruc,
    )
    .map_err(|_| FileSenderErr::TargetDiscovery)
}
