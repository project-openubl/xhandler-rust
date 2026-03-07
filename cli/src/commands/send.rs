use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

#[derive(Args)]
pub struct SendArgs {
    /// Archivo XML firmado a enviar
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo CDR zip de respuesta. Por defecto: <archivo_entrada>.zip
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Usuario SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// Clave SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// Endpoint SOAP de facturas SUNAT (con --beta se usan URLs de prueba)
    #[arg(long = "url-invoice", env = "OPENUBL_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// Endpoint SOAP de percepciones/retenciones SUNAT (con --beta se usan URLs de prueba)
    #[arg(
        long = "url-perception-retention",
        env = "OPENUBL_URL_PERCEPTION_RETENTION"
    )]
    pub url_perception_retention: Option<String>,

    /// Endpoint REST de guias de remision SUNAT (con --beta se usan URLs de prueba)
    #[arg(long = "url-despatch", env = "OPENUBL_URL_DESPATCH")]
    pub url_despatch: Option<String>,

    /// Usar URLs de prueba, credenciales de prueba de SUNAT beta
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

/// Result of sending an XML file to SUNAT.
pub enum SendResult {
    /// CDR was received and saved to the given path.
    Cdr {
        cdr_path: String,
        response_code: String,
        description: String,
        notes: Vec<String>,
    },
    /// A ticket was received (async processing). The ticket may have been verified
    /// interactively if the user accepted.
    Ticket {
        ticket: String,
        verify_result: Option<serde_json::Value>,
    },
    /// SUNAT returned an error.
    Error { code: String, message: String },
}

impl SendArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let xml_content = std::fs::read_to_string(&self.input_file)?;
        let default_cdr = self
            .output_file
            .clone()
            .unwrap_or_else(|| format!("{}.zip", self.input_file));

        let result = self.send_xml_content(&xml_content, &default_cdr).await?;

        match result {
            SendResult::Cdr {
                cdr_path,
                response_code,
                description,
                notes,
            } => {
                let output = serde_json::json!({
                    "cdr": cdr_path,
                    "response_code": response_code,
                    "description": description,
                    "notes": notes,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            SendResult::Ticket {
                ticket,
                verify_result,
            } => {
                let output = serde_json::json!({ "ticket": ticket });
                println!("{}", serde_json::to_string_pretty(&output)?);
                if let Some(verify_output) = verify_result {
                    println!("{}", serde_json::to_string_pretty(&verify_output)?);
                }
                Ok(ExitCode::SUCCESS)
            }
            SendResult::Error { code, message } => {
                let output = serde_json::json!({
                    "code": code,
                    "message": message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }

    /// Sends XML content to SUNAT, handles CDR/ticket/error, and returns a structured result.
    pub async fn send_xml_content(
        &self,
        xml_content: &str,
        default_cdr_path: &str,
    ) -> anyhow::Result<SendResult> {
        let urls = self.resolve_urls();
        let credentials = self.resolve_credentials()?;

        let sender = FileSender { urls, credentials };
        let ubl_file = UblFile {
            file_content: xml_content.to_string(),
        };

        let result = sender.send_file(&ubl_file).await?;

        match result.response {
            SendFileAggregatedResponse::Cdr(cdr_base64, metadata) => {
                use base64::Engine;
                let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&cdr_base64)?;
                let cdr_path = super::absolute_path(default_cdr_path);
                std::fs::write(&cdr_path, cdr_bytes)?;

                Ok(SendResult::Cdr {
                    cdr_path,
                    response_code: metadata.response_code,
                    description: metadata.description,
                    notes: metadata.notes,
                })
            }
            SendFileAggregatedResponse::Ticket(ticket) => {
                let verify_result =
                    super::prompt_verify_ticket(&ticket, &sender, self.beta, default_cdr_path)
                        .await?;

                Ok(SendResult::Ticket {
                    ticket,
                    verify_result,
                })
            }
            SendFileAggregatedResponse::Error(error) => Ok(SendResult::Error {
                code: error.code,
                message: error.message,
            }),
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
