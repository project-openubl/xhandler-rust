use serde::Serialize;

use xhandler::prelude::*;

use super::send::SendConfig;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum VerifyResponse {
    Cdr {
        cdr_base64: String,
        status_code: String,
        response_code: String,
        description: String,
        notes: Vec<String>,
    },
    Error {
        code: String,
        message: String,
    },
}

#[tauri::command]
pub async fn verify_ticket(ticket: String, config: SendConfig) -> Result<VerifyResponse, String> {
    let urls = config.resolve_urls();
    let credentials = config.resolve_credentials()?;

    let sender = FileSender { urls, credentials };
    let target = VerifyTicketTarget::Soap(config.url_invoice.clone().unwrap_or_else(|| {
        if config.beta {
            "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string()
        } else {
            "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService".to_string()
        }
    }));

    let result = sender
        .verify_ticket(&target, &ticket)
        .await
        .map_err(|e| format!("Error al verificar ticket: {e}"))?;

    match result.response {
        VerifyTicketAggregatedResponse::Cdr(status, metadata) => Ok(VerifyResponse::Cdr {
            cdr_base64: status.cdr_base64,
            status_code: status.status_code,
            response_code: metadata.response_code,
            description: metadata.description,
            notes: metadata.notes,
        }),
        VerifyTicketAggregatedResponse::Error(error) => Ok(VerifyResponse::Error {
            code: error.code,
            message: error.message,
        }),
    }
}
