use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::commands::send::SendArgs;

#[derive(Args)]
pub struct VerifyTicketArgs {
    /// Ticket number from a previous send response
    #[arg(long)]
    pub ticket: String,

    /// Output file for CDR XML response
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// SUNAT SOL username (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// SUNAT SOL password (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// SUNAT invoice SOAP endpoint (used for ticket verification)
    #[arg(long = "url-invoice", env = "OPENUBL_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// Use SUNAT beta/test environment (URLs and credentials)
    #[arg(long)]
    pub beta: bool,
}

impl VerifyTicketArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let send_args = SendArgs {
            input_file: String::new(),
            output_file: self.output_file.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            url_invoice: self.url_invoice.clone(),
            url_perception_retention: None,
            url_despatch: None,
            beta: self.beta,
        };

        let urls = send_args.resolve_urls();
        let credentials = send_args.resolve_credentials()?;

        let sender = FileSender { urls, credentials };
        let target = VerifyTicketTarget::Soap(self.url_invoice.clone().unwrap_or_else(|| {
            if self.beta {
                "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string()
            } else {
                "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService".to_string()
            }
        }));

        let result = sender.verify_ticket(&target, &self.ticket).await?;

        match result.response {
            VerifyTicketAggregatedResponse::Cdr(status, metadata) => {
                if let Some(path) = &self.output_file {
                    use base64::Engine;
                    let cdr_bytes =
                        base64::engine::general_purpose::STANDARD.decode(&status.cdr_base64)?;
                    std::fs::write(path, cdr_bytes)?;
                }
                let output = serde_json::json!({
                    "status_code": status.status_code,
                    "response_code": metadata.response_code,
                    "description": metadata.description,
                    "notes": metadata.notes,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            VerifyTicketAggregatedResponse::Error(error) => {
                let output = serde_json::json!({
                    "code": error.code,
                    "message": error.message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }
}
