use std::process::ExitCode;

use anyhow::Context;
use clap::Args;

use xhandler::prelude::*;

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  openubl send -f firmado.xml --beta
  openubl send -f firmado.xml --username 20123456789MODDATOS --password clave
  openubl send -f guia.xml --username 20123456789USUARIO --password clave --client-id id --client-secret secreto

La salida es JSON en stdout. Codigo de salida: 0=aceptado/ticket, 2=rechazado o error SUNAT.")]
pub struct SendArgs {
    /// Archivo XML firmado a enviar
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo CDR zip de respuesta. Por defecto: <archivo_entrada>.zip
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    #[command(flatten)]
    pub credentials: CredentialArgs,

    /// Endpoint SOAP de facturas SUNAT (con --beta se usan URLs de prueba)
    #[arg(long = "url-invoice", env = "OPENUBL_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// Endpoint SOAP de percepciones/retenciones SUNAT (con --beta se usan URLs de prueba)
    #[arg(
        long = "url-perception-retention",
        env = "OPENUBL_URL_PERCEPTION_RETENTION"
    )]
    pub url_perception_retention: Option<String>,

    /// Endpoint REST de guias de remision SUNAT (requerido con --beta: SUNAT no tiene beta para guias)
    #[arg(long = "url-despatch", env = "OPENUBL_URL_DESPATCH")]
    pub url_despatch: Option<String>,

    /// Usar URLs de prueba, credenciales de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,

    /// Desactivar prompts interactivos (util para scripts)
    #[arg(long)]
    pub no_interactive: bool,
}

const BETA_URL_INVOICE: &str = "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService";
const BETA_URL_PERCEPTION_RETENTION: &str =
    "https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService";

const PROD_URL_INVOICE: &str = "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService";
const PROD_URL_PERCEPTION_RETENTION: &str =
    "https://e-factura.sunat.gob.pe/ol-ti-itemision-otroscpe-gem/billService";
/// SUNAT has no beta environment for guias de remision
const URL_DESPATCH: &str = "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem";

const BETA_USERNAME: &str = "12345678959MODDATOS";
const BETA_PASSWORD: &str = "MODDATOS";

/// SUNAT credentials shared by send, apply and verify-ticket
#[derive(Args, Clone, Default)]
pub struct CredentialArgs {
    /// Usuario SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// Clave SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// Client ID de la API REST de SUNAT (requerido para guias de remision)
    #[arg(long = "client-id", env = "OPENUBL_CLIENT_ID")]
    pub client_id: Option<String>,

    /// Client secret de la API REST de SUNAT (requerido para guias de remision)
    #[arg(long = "client-secret", env = "OPENUBL_CLIENT_SECRET")]
    pub client_secret: Option<String>,
}

impl CredentialArgs {
    pub fn resolve(&self, beta: bool) -> anyhow::Result<Credentials> {
        let username = self
            .username
            .clone()
            .or_else(|| beta.then(|| BETA_USERNAME.to_string()))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "se requiere --username (o usar --beta para credenciales de prueba)"
                )
            })?;
        let password = self
            .password
            .clone()
            .or_else(|| beta.then(|| BETA_PASSWORD.to_string()))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "se requiere --password (o usar --beta para credenciales de prueba)"
                )
            })?;
        Ok(Credentials {
            username,
            password,
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
        })
    }
}

/// Result of sending an XML file to SUNAT.
pub enum SendResult {
    /// CDR was received and saved to the given path.
    Cdr {
        cdr_path: String,
        metadata: CdrMetadata,
    },
    /// A ticket was received (async processing), with the verification output
    /// if the user verified it interactively.
    Ticket {
        ticket: String,
        verification: Option<(serde_json::Value, Status)>,
    },
    /// SUNAT returned an error (SOAP fault or REST error).
    Error(ErrorResponse),
}

impl SendResult {
    /// Prints the result as JSON, adding the `extra` fields, and returns the exit code.
    /// SUNAT errors go to stderr, everything else to stdout.
    pub fn print(self, extra: &[(&str, serde_json::Value)]) -> anyhow::Result<ExitCode> {
        let is_fault = matches!(self, SendResult::Error(_));
        let (mut output, status) = match self {
            SendResult::Cdr { cdr_path, metadata } => (
                serde_json::json!({
                    "cdr": cdr_path,
                    "response_code": metadata.response_code,
                    "status": metadata.status().to_string(),
                    "description": metadata.description,
                    "notes": metadata.notes,
                }),
                metadata.status(),
            ),
            SendResult::Ticket {
                ticket,
                verification,
            } => {
                let output = serde_json::json!({ "ticket": ticket });
                match verification {
                    Some((verify_output, status)) => {
                        println!("{}", serde_json::to_string_pretty(&output)?);
                        (verify_output, status)
                    }
                    None => (output, Status::EnProceso),
                }
            }
            SendResult::Error(error) => (
                serde_json::json!({
                    "code": error.code,
                    "sunat_code": error.sunat_code(),
                    "status": error.status().to_string(),
                    "message": error.message,
                }),
                error.status(),
            ),
        };

        for (key, value) in extra {
            output[*key] = value.clone();
        }

        let output = serde_json::to_string_pretty(&output)?;
        if is_fault {
            eprintln!("{output}");
        } else {
            println!("{output}");
        }
        Ok(super::exit_code_for(status))
    }
}

impl SendArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        // Read raw bytes: signed XML may be ISO-8859-1 and must reach SUNAT unchanged
        let xml_bytes = std::fs::read(&self.input_file)
            .with_context(|| format!("no se puede leer el archivo XML: {}", self.input_file))?;
        let ubl_file = UblFile::from_bytes(xml_bytes).with_context(|| {
            format!(
                "no se puede decodificar el archivo XML: {}",
                self.input_file
            )
        })?;
        let default_cdr = self
            .output_file
            .clone()
            .unwrap_or_else(|| format!("{}.zip", self.input_file));

        self.send_ubl_file(&ubl_file, &default_cdr)
            .await?
            .print(&[])
    }

    /// Sends a UBL file to SUNAT, handles CDR/ticket/error, and returns a structured result.
    pub async fn send_ubl_file(
        &self,
        ubl_file: &UblFile,
        default_cdr_path: &str,
    ) -> anyhow::Result<SendResult> {
        let is_despatch = || {
            ubl_file
                .metadata()
                .is_ok_and(|metadata| metadata.document_type == DocumentType::DESPATCH_ADVICE)
        };
        if self.beta && is_despatch() {
            self.check_despatch_beta()?;
        }

        let urls = self.resolve_urls();
        let credentials = self.credentials.resolve(self.beta)?;

        let sender = FileSender { urls, credentials };

        let result = sender
            .send_file(ubl_file)
            .await
            .map_err(|e| anyhow::anyhow!("error al enviar documento a SUNAT: {e}"))?;

        match result.response {
            SendFileAggregatedResponse::Cdr(cdr_base64, metadata) => {
                use base64::Engine;
                let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&cdr_base64)?;
                let cdr_path = super::absolute_path(default_cdr_path);
                std::fs::write(&cdr_path, cdr_bytes)?;

                Ok(SendResult::Cdr { cdr_path, metadata })
            }
            SendFileAggregatedResponse::Ticket(ticket) => {
                let verification = if self.no_interactive {
                    None
                } else {
                    // Fall back to the invoice service, which handles most tickets
                    let target = result
                        .verify_ticket_target
                        .unwrap_or_else(|| VerifyTicketTarget::Soap(sender.urls.invoice.clone()));
                    super::prompt_verify_ticket(&ticket, &sender, &target, default_cdr_path).await?
                };

                Ok(SendResult::Ticket {
                    ticket,
                    verification,
                })
            }
            SendFileAggregatedResponse::Error(error) => Ok(SendResult::Error(error)),
        }
    }

    /// SUNAT has no beta environment for guias de remision, so `--beta` would
    /// send them to production. Require an explicit `--url-despatch` instead.
    pub fn check_despatch_beta(&self) -> anyhow::Result<()> {
        if self.beta && self.url_despatch.is_none() {
            anyhow::bail!(
                "SUNAT no tiene ambiente beta para guias de remision; usar --url-despatch para indicar el endpoint"
            );
        }
        Ok(())
    }

    pub fn resolve_urls(&self) -> Urls {
        let (default_invoice, default_pr) = if self.beta {
            (BETA_URL_INVOICE, BETA_URL_PERCEPTION_RETENTION)
        } else {
            (PROD_URL_INVOICE, PROD_URL_PERCEPTION_RETENTION)
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
                .unwrap_or_else(|| URL_DESPATCH.to_string()),
        }
    }
}
