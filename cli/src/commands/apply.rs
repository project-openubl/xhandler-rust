use std::process::ExitCode;

use clap::Args;

use crate::commands::create::create_xml;
use xhandler::prelude::{UblFile, XSigner};

use crate::commands::send::{CredentialArgs, SendArgs};
use crate::commands::sign::KeyArgs;

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  openubl apply -f factura.yaml --beta
  openubl apply -f factura.yaml --beta --save-xml --dry-run
  openubl apply -f factura.yaml --private-key llave.pem --certificate cert.pem --username user --password clave
  openubl apply -f factura.yaml --pfx certificado.pfx --pfx-password clave --username user --password clave

La salida es JSON en stdout. Codigo de salida: 0=aceptado/ticket, 2=rechazado o error SUNAT.")]
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

    #[command(flatten)]
    pub keys: KeyArgs,

    #[command(flatten)]
    pub credentials: CredentialArgs,

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

    /// Desactivar prompts interactivos (util para scripts)
    #[arg(long)]
    pub no_interactive: bool,
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
        let key_pair = self.keys.resolve(self.beta)?;
        let signed_xml = XSigner { xml_document: xml }.sign(&key_pair)?;

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
        let ubl_file = UblFile::from_bytes(signed_xml)?;
        let default_cdr = self
            .output_file
            .clone()
            .unwrap_or_else(|| format!("{}.cdr.zip", self.input_file));

        let send_args = SendArgs {
            input_file: String::new(),
            output_file: self.output_file.clone(),
            credentials: self.credentials.clone(),
            url_invoice: None,
            url_perception_retention: None,
            url_despatch: None,
            beta: self.beta,
            no_interactive: self.no_interactive,
        };

        send_args
            .send_ubl_file(&ubl_file, &default_cdr)
            .await?
            .print(&[
                ("unsigned_xml", serde_json::json!(unsigned_path)),
                ("signed_xml", serde_json::json!(signed_path)),
            ])
    }
}
