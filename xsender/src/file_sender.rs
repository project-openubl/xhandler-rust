use anyhow::anyhow;
use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::client_sunat::{
    ClientSUNAT, ClientSunatErr, ErrorResponse, File, SendFileResponse, VerifyTicketResponse,
};
use crate::models::{Credentials, SendFileTarget, Urls, VerifyTicketTarget};
use crate::prelude::VerifyTicketStatus;
use crate::soap::cdr::CdrMetadata;
use crate::ubl_file::UblFile;
use crate::zip_manager::{create_zip_from_str, decode_base64_zip_and_extract_first_file};

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
    Error(ErrorResponse),
}

impl FileSender {
    pub async fn send_file(&self, xml: &UblFile) -> Result<SendFileResponseWrapper, FileSenderErr> {
        let metadata = xml.metadata()?;

        let filename_without_extension = filename_formatted_without_extension(
            &metadata.document_type,
            &metadata.document_id,
            &metadata.ruc,
        )
        .map_err(|_| FileSenderErr::TargetDiscovery)?;

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

        let zip_file_name = format!("{filename_without_extension}.zip");
        let file_name_inside_zip = format!("{filename_without_extension}.xml");

        let zip = create_zip_from_str(&(xml.file_content), &file_name_inside_zip)?;
        let zip_base64 = general_purpose::STANDARD.encode(zip);

        let file_to_be_sent = File {
            name: zip_file_name,
            base_64: zip_base64,
        };

        let client = ClientSUNAT::default();
        let result = client
            .send_file(&send_target, &file_to_be_sent, &self.credentials)
            .await?;

        let response = match result {
            SendFileResponse::Cdr(cdr_base64) => {
                let cdr_xml = decode_base64_zip_and_extract_first_file(&cdr_base64)?.ok_or(
                    FileSenderErr::Any(anyhow!("Could not extract the first file from zip")),
                )?;
                let cdr_metadata = CdrMetadata::from_str(&cdr_xml)?;
                SendFileAggregatedResponse::Cdr(cdr_base64, cdr_metadata)
            }
            SendFileResponse::Ticket(ticket) => SendFileAggregatedResponse::Ticket(ticket),
            SendFileResponse::Error(error) => SendFileAggregatedResponse::Error(ErrorResponse {
                code: error.code,
                message: error.message,
            }),
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
        let client = ClientSUNAT::default();
        let result = client
            .verify_ticket(target, ticket, &self.credentials)
            .await?;

        let response = match result {
            VerifyTicketResponse::Cdr(status) => {
                let cdr_xml = decode_base64_zip_and_extract_first_file(&status.cdr_base64)?.ok_or(
                    FileSenderErr::Any(anyhow!("Could not extract the first file from zip")),
                )?;
                let cdr_metadata = CdrMetadata::from_str(&cdr_xml)?;
                VerifyTicketAggregatedResponse::Cdr(status, cdr_metadata)
            }
            VerifyTicketResponse::Error(error) => {
                VerifyTicketAggregatedResponse::Error(ErrorResponse {
                    code: error.code,
                    message: error.message,
                })
            }
        };

        Ok(VerifyTicketResponseWrapper { response })
    }
}
