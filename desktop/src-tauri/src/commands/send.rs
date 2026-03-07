use serde::{Deserialize, Serialize};

use xhandler::prelude::*;

const BETA_URL_INVOICE: &str = "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService";
const BETA_URL_PERCEPTION_RETENTION: &str =
    "https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService";
const BETA_URL_DESPATCH: &str = "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem";

const BETA_USERNAME: &str = "12345678959MODDATOS";
const BETA_PASSWORD: &str = "MODDATOS";

#[derive(Debug, Deserialize)]
pub struct SendConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub url_invoice: Option<String>,
    pub url_perception_retention: Option<String>,
    pub url_despatch: Option<String>,
    pub beta: bool,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum SendResponse {
    Cdr {
        cdr_base64: String,
        response_code: String,
        description: String,
        notes: Vec<String>,
    },
    Ticket {
        ticket: String,
    },
    Error {
        code: String,
        message: String,
    },
}

impl SendConfig {
    pub(crate) fn resolve_credentials(&self) -> Result<Credentials, String> {
        let username = self
            .username
            .clone()
            .or_else(|| self.beta.then(|| BETA_USERNAME.to_string()))
            .ok_or("Se requiere username")?;
        let password = self
            .password
            .clone()
            .or_else(|| self.beta.then(|| BETA_PASSWORD.to_string()))
            .ok_or("Se requiere password")?;
        Ok(Credentials { username, password })
    }

    pub(crate) fn resolve_urls(&self) -> Urls {
        let (default_invoice, default_pr, default_despatch) = if self.beta {
            (
                BETA_URL_INVOICE,
                BETA_URL_PERCEPTION_RETENTION,
                BETA_URL_DESPATCH,
            )
        } else {
            (
                "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService",
                "https://e-factura.sunat.gob.pe/ol-ti-itemision-otroscpe-gem/billService",
                "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem",
            )
        };

        Urls {
            invoice: self
                .url_invoice
                .clone()
                .unwrap_or_else(|| default_invoice.to_string()),
            perception_retention: self
                .url_perception_retention
                .clone()
                .unwrap_or_else(|| default_pr.to_string()),
            despatch: self
                .url_despatch
                .clone()
                .unwrap_or_else(|| default_despatch.to_string()),
        }
    }
}

#[tauri::command]
pub async fn send_xml(signed_xml: String, config: SendConfig) -> Result<SendResponse, String> {
    let urls = config.resolve_urls();
    let credentials = config.resolve_credentials()?;

    let sender = FileSender { urls, credentials };
    let ubl_file = UblFile {
        file_content: signed_xml,
    };

    let result = sender
        .send_file(&ubl_file)
        .await
        .map_err(|e| format!("Error al enviar documento a SUNAT: {e}"))?;

    match result.response {
        SendFileAggregatedResponse::Cdr(cdr_base64, metadata) => Ok(SendResponse::Cdr {
            cdr_base64,
            response_code: metadata.response_code,
            description: metadata.description,
            notes: metadata.notes,
        }),
        SendFileAggregatedResponse::Ticket(ticket) => Ok(SendResponse::Ticket { ticket }),
        SendFileAggregatedResponse::Error(error) => Ok(SendResponse::Error {
            code: error.code,
            message: error.message,
        }),
    }
}
