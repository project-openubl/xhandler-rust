use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

#[derive(Args)]
pub struct SendArgs {
    /// Signed XML file to send
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Output file for CDR zip response. Defaults to <input_file>.zip
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// SUNAT SOL username (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// SUNAT SOL password (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// SUNAT invoice SOAP endpoint (defaults to beta urls when not defined)
    #[arg(long = "url-invoice", env = "OPENUBL_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// SUNAT perception/retention SOAP endpoint (defaults to beta urls when not defined)
    #[arg(
        long = "url-perception-retention",
        env = "OPENUBL_URL_PERCEPTION_RETENTION"
    )]
    pub url_perception_retention: Option<String>,

    /// SUNAT despatch REST endpoint (defaults to beta urls when not defined)
    #[arg(long = "url-despatch", env = "OPENUBL_URL_DESPATCH")]
    pub url_despatch: Option<String>,

    /// Use SUNAT beta URLs and test credentials
    #[arg(long)]
    pub beta: bool,
}

const BETA_URL_INVOICE: &str = "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService";
const BETA_URL_PERCEPTION_RETENTION: &str =
    "https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService";
const BETA_URL_DESPATCH: &str = "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem";

const PROD_URL_INVOICE: &str = "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService";
const PROD_URL_PERCEPTION_RETENTION: &str =
    "https://e-factura.sunat.gob.pe/ol-ti-itemision-otroscpe-gem/billService";
const PROD_URL_DESPATCH: &str = "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem";

const BETA_USERNAME: &str = "12345678959MODDATOS";
const BETA_PASSWORD: &str = "MODDATOS";

impl SendArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let xml_content = std::fs::read_to_string(&self.input_file)?;

        let urls = self.resolve_urls();
        let credentials = self.resolve_credentials()?;

        let sender = FileSender { urls, credentials };
        let ubl_file = UblFile {
            file_content: xml_content,
        };

        let result = sender.send_file(&ubl_file).await?;

        match result.response {
            SendFileAggregatedResponse::Cdr(cdr_base64, metadata) => {
                let cdr_path = self
                    .output_file
                    .clone()
                    .unwrap_or_else(|| format!("{}.zip", self.input_file));

                use base64::Engine;
                let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&cdr_base64)?;
                std::fs::write(&cdr_path, cdr_bytes)?;

                let output = serde_json::json!({
                    "cdr": cdr_path,
                    "response_code": metadata.response_code,
                    "description": metadata.description,
                    "notes": metadata.notes,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            SendFileAggregatedResponse::Ticket(ticket) => {
                let output = serde_json::json!({ "ticket": ticket });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            SendFileAggregatedResponse::Error(error) => {
                let output = serde_json::json!({
                    "code": error.code,
                    "message": error.message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }

    pub fn resolve_credentials(&self) -> anyhow::Result<Credentials> {
        let username = self
            .username
            .clone()
            .or_else(|| self.beta.then(|| BETA_USERNAME.to_string()))
            .ok_or_else(|| {
                anyhow::anyhow!("--username is required (or use --beta for test credentials)")
            })?;
        let password = self
            .password
            .clone()
            .or_else(|| self.beta.then(|| BETA_PASSWORD.to_string()))
            .ok_or_else(|| {
                anyhow::anyhow!("--password is required (or use --beta for test credentials)")
            })?;
        Ok(Credentials { username, password })
    }

    pub fn resolve_urls(&self) -> Urls {
        let (default_invoice, default_pr, default_despatch) = if self.beta {
            (
                BETA_URL_INVOICE,
                BETA_URL_PERCEPTION_RETENTION,
                BETA_URL_DESPATCH,
            )
        } else {
            (
                PROD_URL_INVOICE,
                PROD_URL_PERCEPTION_RETENTION,
                PROD_URL_DESPATCH,
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
