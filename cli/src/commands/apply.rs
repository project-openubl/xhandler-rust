use std::process::ExitCode;

use clap::Args;

use crate::commands::create::CreateArgs;
use crate::commands::send::SendArgs;
use crate::commands::sign::SignArgs;
use crate::input::{self, DocumentInput};

use xhandler::prelude::*;

#[derive(Args)]
pub struct ApplyArgs {
    /// Input JSON/YAML document definition
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Output file for CDR zip response. Defaults to <input_file>.cdr.zip
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Input format when reading from stdin: json, yaml
    #[arg(long = "format")]
    pub format: Option<String>,

    /// Path to PKCS#1 PEM private key file (defaults to test key when --beta is used)
    #[arg(long = "private-key", env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: Option<String>,

    /// Path to X.509 PEM certificate file (defaults to test cert when --beta is used)
    #[arg(long = "certificate", env = "OPENUBL_CERTIFICATE")]
    pub certificate: Option<String>,

    /// SUNAT SOL username (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// SUNAT SOL password (defaults to beta credentials when --beta is used)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// Use SUNAT beta URLs, test credentials, and test certificates
    #[arg(long)]
    pub beta: bool,

    /// Save unsigned XML. Defaults to <input_file>.unsigned.xml
    #[arg(long = "save-xml", num_args = 0..=1, default_missing_value = "")]
    pub save_xml: Option<String>,

    /// Signed XML output path. Defaults to <input_file>.signed.xml
    #[arg(long = "save-signed-xml")]
    pub save_signed_xml: Option<String>,

    /// Run create + sign but do not send
    #[arg(long)]
    pub dry_run: bool,
}

impl ApplyArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        // Resolve output paths to absolute
        let unsigned_path = self.save_xml.as_ref().map(|p| {
            let path = if p.is_empty() {
                format!("{}.unsigned.xml", self.input_file)
            } else {
                p.clone()
            };
            super::absolute_path(&path)
        });
        let signed_path = super::absolute_path(
            &self
                .save_signed_xml
                .clone()
                .unwrap_or_else(|| format!("{}.signed.xml", self.input_file)),
        );

        // Step 1: Create XML
        let create_args = CreateArgs {
            input_file: self.input_file.clone(),
            output_file: None,
            format: self.format.clone(),
            dry_run: false,
        };

        let doc = input::read_input(&create_args.input_file, create_args.format.as_deref())?;

        let defaults = Defaults {
            icb_tasa: rust_decimal_macros::dec!(0.2),
            igv_tasa: rust_decimal_macros::dec!(0.18),
            ivap_tasa: rust_decimal_macros::dec!(0.04),
            date: chrono::Local::now().date_naive(),
        };

        let xml = match doc {
            DocumentInput::Invoice { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::CreditNote { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::DebitNote { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::DespatchAdvice { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::Perception { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::Retention { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::SummaryDocuments { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
            DocumentInput::VoidedDocuments { mut spec } => {
                spec.enrich(&defaults);
                spec.render()?
            }
        };

        if let Some(path) = &unsigned_path {
            std::fs::write(path, &xml)?;
        }

        // Step 2: Sign
        let sign_args = SignArgs {
            input_file: String::new(),
            output_file: None,
            private_key: self.private_key.clone(),
            certificate: self.certificate.clone(),
            beta: self.beta,
        };

        let key_pair = sign_args.resolve_key_pair()?;
        let signed_xml = sign_args.sign_xml(&xml, &key_pair)?;

        std::fs::write(&signed_path, &signed_xml)?;

        if self.dry_run {
            let output = serde_json::json!({
                "unsigned_xml": unsigned_path,
                "signed_xml": signed_path,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(ExitCode::SUCCESS);
        }

        // Step 3: Send
        let signed_xml_str = String::from_utf8(signed_xml)?;
        let send_args = SendArgs {
            input_file: String::new(),
            output_file: self.output_file.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            url_invoice: None,
            url_perception_retention: None,
            url_despatch: None,
            beta: self.beta,
        };

        let urls = send_args.resolve_urls();
        let credentials = send_args.resolve_credentials()?;

        let sender = FileSender { urls, credentials };
        let ubl_file = UblFile {
            file_content: signed_xml_str,
        };

        let result = sender.send_file(&ubl_file).await?;

        match result.response {
            SendFileAggregatedResponse::Cdr(cdr_base64, metadata) => {
                let cdr_path = super::absolute_path(
                    &self
                        .output_file
                        .clone()
                        .unwrap_or_else(|| format!("{}.cdr.zip", self.input_file)),
                );

                use base64::Engine;
                let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&cdr_base64)?;
                std::fs::write(&cdr_path, cdr_bytes)?;

                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
                    "cdr": cdr_path,
                    "response_code": metadata.response_code,
                    "description": metadata.description,
                    "notes": metadata.notes,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            SendFileAggregatedResponse::Ticket(ticket) => {
                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
                    "ticket": ticket,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            SendFileAggregatedResponse::Error(error) => {
                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
                    "code": error.code,
                    "message": error.message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }
}
