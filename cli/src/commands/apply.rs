use std::process::ExitCode;

use clap::Args;

use crate::commands::create::create_xml;
use crate::commands::send::{SendArgs, SendResult};
use crate::commands::sign::SignArgs;

#[derive(Args)]
pub struct ApplyArgs {
    /// Archivo de entrada JSON/YAML con la definicion del documento
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo CDR zip de respuesta. Por defecto: <archivo_entrada>.cdr.zip
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Formato de entrada cuando se lee desde stdin: json, yaml
    #[arg(long = "format")]
    pub format: Option<String>,

    /// Ruta al archivo de llave privada PKCS#1 PEM (con --beta se usan certificados de prueba)
    #[arg(long = "private-key", env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: Option<String>,

    /// Ruta al archivo de certificado X.509 PEM (con --beta se usan certificados de prueba)
    #[arg(long = "certificate", env = "OPENUBL_CERTIFICATE")]
    pub certificate: Option<String>,

    /// Usuario SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// Clave SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// Usar URLs de prueba, credenciales y certificados de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,

    /// Guardar XML sin firmar. Por defecto: <archivo_entrada>.unsigned.xml
    #[arg(long = "save-xml", num_args = 0..=1, default_missing_value = "")]
    pub save_xml: Option<String>,

    /// Ruta del XML firmado. Por defecto: <archivo_entrada>.signed.xml
    #[arg(long = "save-signed-xml")]
    pub save_signed_xml: Option<String>,

    /// Ejecutar crear + firmar sin enviar a SUNAT
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
        let xml = create_xml(&self.input_file, self.format.as_deref())?;

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
        let default_cdr = self
            .output_file
            .clone()
            .unwrap_or_else(|| format!("{}.cdr.zip", self.input_file));

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

        let result = send_args
            .send_xml_content(&signed_xml_str, &default_cdr)
            .await?;

        match result {
            SendResult::Cdr {
                cdr_path,
                response_code,
                description,
                notes,
            } => {
                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
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
                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
                    "ticket": ticket,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                if let Some(verify_output) = verify_result {
                    println!("{}", serde_json::to_string_pretty(&verify_output)?);
                }
                Ok(ExitCode::SUCCESS)
            }
            SendResult::Error { code, message } => {
                let output = serde_json::json!({
                    "unsigned_xml": unsigned_path,
                    "signed_xml": signed_path,
                    "code": code,
                    "message": message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }
}
