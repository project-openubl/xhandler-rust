use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::client_sunat::{
    ClientSUNAT, ErrorClientSUNAT, ErrorResponse, File, SendFileResponse, VerifyTicketResponse,
};
use crate::models::{Credentials, SendFileTarget, Urls, VerifyTicketTarget};
use crate::prelude::VerifyTicketStatus;
use crate::soap::cdr::{CdrMetadata, CdrReadError};
use crate::ubl_file::{UblFile, UblMetadataError};
use crate::zip_manager::{create_zip, extract_cdr_from_base64_zip, ExtractCdrFromBase64ZipError};

pub struct FileSender {
    pub urls: Urls,
    pub credentials: Credentials,
}

#[derive(Debug)]
pub struct FileSenderError {
    pub message: String,
    pub client_error: Option<ErrorClientSUNAT>,
}

pub struct SendFileResponseWrapper {
    pub response: SendFileAggregatedResponse,
    pub send_file_target: SendFileTarget,
    pub verify_ticket_target: Option<VerifyTicketTarget>,
}

pub enum SendFileAggregatedResponse {
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
    pub async fn send_file(
        &self,
        xml: &UblFile,
    ) -> Result<SendFileResponseWrapper, FileSenderError> {
        let metadata = xml.metadata()?;

        let filename_without_extension = filename_formatted_without_extension(
            &metadata.document_type,
            &metadata.document_id,
            &metadata.ruc,
        )
        .ok_or(FileSenderError {
            message: "Could not determine the filename of the file to be sent".to_string(),
            client_error: None,
        })?;

        let send_target = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        )
        .ok_or(FileSenderError {
            message: "Could not determine the target for the file to be sent".to_string(),
            client_error: None,
        })?;
        let verify_ticket_target = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &self.urls,
        );

        let zip = create_zip(&filename_without_extension, &xml.file_content.clone())?;
        let zip_base64 = general_purpose::STANDARD.encode(zip);

        let file_to_be_sent = File {
            name: format!("{filename_without_extension}.zip"),
            base_64: zip_base64,
        };

        let client = ClientSUNAT::default();
        let result = client
            .send_file(&send_target, &file_to_be_sent, &self.credentials)
            .await?;

        let response = match result {
            SendFileResponse::Cdr(cdr_base64) => {
                let cdr_xml = extract_cdr_from_base64_zip(&cdr_base64)?;
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
    ) -> Result<VerifyTicketResponseWrapper, FileSenderError> {
        let client = ClientSUNAT::default();
        let result = client
            .verify_ticket(target, ticket, &self.credentials)
            .await?;

        let response = match result {
            VerifyTicketResponse::Cdr(status) => {
                let cdr_xml = extract_cdr_from_base64_zip(&status.cdr_base64)?;
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

impl From<UblMetadataError> for FileSenderError {
    fn from(value: UblMetadataError) -> Self {
        Self {
            message: value.message.to_string(),
            client_error: None,
        }
    }
}

impl From<ZipError> for FileSenderError {
    fn from(value: ZipError) -> Self {
        Self {
            message: value.to_string(),
            client_error: None,
        }
    }
}

impl From<ErrorClientSUNAT> for FileSenderError {
    fn from(value: ErrorClientSUNAT) -> Self {
        Self {
            message: "Error while sending the file".to_string(),
            client_error: Some(value),
        }
    }
}

impl From<ExtractCdrFromBase64ZipError> for FileSenderError {
    fn from(_: ExtractCdrFromBase64ZipError) -> Self {
        Self {
            message: "Error extracting CDR from base64".to_string(),
            client_error: None,
        }
    }
}

impl From<CdrReadError> for FileSenderError {
    fn from(_: CdrReadError) -> Self {
        Self {
            message: "Error while reading CDR".to_string(),
            client_error: None,
        }
    }
}
