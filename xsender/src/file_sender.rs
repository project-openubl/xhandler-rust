use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipError;

use crate::analyzer::{
    filename_formatted_without_extension, send_file_target, verify_ticket_target,
};
use crate::client_sunat::{
    ClientSUNAT, ErrorClientSUNAT, File, SendFileResponse, VerifyTicketResponse,
};
use crate::models::{Credentials, SendFileTarget, Urls, VerifyTicketTarget};
use crate::ubl_file::{UblFile, UblMetadataError};
use crate::zip_manager::create_zip;

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
    pub data: SendFileResponse,
    pub send_file_target: SendFileTarget,
    pub verify_ticket_target: Option<VerifyTicketTarget>,
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

        Ok(SendFileResponseWrapper {
            data: result,
            send_file_target: send_target,
            verify_ticket_target,
        })
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
    ) -> Result<VerifyTicketResponse, ErrorClientSUNAT> {
        let client = ClientSUNAT::default();
        client
            .verify_ticket(target, ticket, &self.credentials)
            .await
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
