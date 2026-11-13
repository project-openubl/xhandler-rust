use std::io::Read;
use std::process::ExitCode;

use anyhow::Context;
use clap::Args;

use xhandler::prelude::*;

pub const BETA_PRIVATE_KEY: &str = include_str!("../../../xsigner/resources/test/private.key");
pub const BETA_CERTIFICATE: &str = include_str!("../../../xsigner/resources/test/public.cer");

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  openubl sign -f factura.xml --beta
  openubl sign -f factura.xml --private-key llave.pem --certificate cert.pem -o firmado.xml
  openubl sign -f factura.xml --pfx certificado.pfx --pfx-password clave -o firmado.xml")]
pub struct SignArgs {
    /// Archivo XML sin firmar. Usar "-" para leer desde stdin
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo XML firmado. Si se omite, se imprime en stdout
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Ruta al archivo de llave privada PEM, PKCS#1 o PKCS#8 (con --beta se usan certificados de prueba)
    #[arg(long = "private-key", env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: Option<String>,

    /// Ruta al archivo de certificado X.509 PEM (con --beta se usan certificados de prueba)
    #[arg(long = "certificate", env = "OPENUBL_CERTIFICATE")]
    pub certificate: Option<String>,

    /// Ruta al certificado PKCS#12 (.pfx/.p12), en lugar de --private-key y --certificate
    #[arg(long = "pfx", env = "OPENUBL_PFX", conflicts_with_all = ["private_key", "certificate"])]
    pub pfx: Option<String>,

    /// Clave del certificado PKCS#12
    #[arg(long = "pfx-password", env = "OPENUBL_PFX_PASSWORD", requires = "pfx")]
    pub pfx_password: Option<String>,

    /// Usar certificados de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,
}

impl SignArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let xml_content = if self.input_file == "-" {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        } else {
            // Accepts UTF-8 and ISO-8859-1 files
            XSigner::from_file(&self.input_file)
                .with_context(|| format!("no se puede leer el archivo XML: {}", self.input_file))?
                .xml_document
        };

        let key_pair = self.resolve_key_pair()?;
        let signed_xml = self.sign_xml(&xml_content, &key_pair)?;

        match &self.output_file {
            Some(path) => std::fs::write(path, &signed_xml)?,
            None => {
                let output = String::from_utf8(signed_xml)?;
                print!("{output}");
            }
        }

        Ok(ExitCode::SUCCESS)
    }

    pub fn resolve_key_pair(&self) -> anyhow::Result<RsaKeyPair> {
        if let Some(path) = &self.pfx {
            let pfx = std::fs::read(path)
                .with_context(|| format!("no se puede leer el certificado PKCS#12: {path}"))?;
            let password = self.pfx_password.as_deref().unwrap_or_default();
            return RsaKeyPair::from_pkcs12(&pfx, password)
                .with_context(|| format!("error al leer el certificado PKCS#12: {path}"));
        }

        let private_key_pem = match &self.private_key {
            Some(path) => std::fs::read_to_string(path)
                .with_context(|| format!("no se puede leer la llave privada: {path}"))?,
            None if self.beta => BETA_PRIVATE_KEY.to_string(),
            None => {
                anyhow::bail!(
                    "se requiere --private-key (o usar --beta para certificados de prueba)"
                )
            }
        };
        let certificate_pem = match &self.certificate {
            Some(path) => std::fs::read_to_string(path)
                .with_context(|| format!("no se puede leer el certificado: {path}"))?,
            None if self.beta => BETA_CERTIFICATE.to_string(),
            None => {
                anyhow::bail!(
                    "se requiere --certificate (o usar --beta para certificados de prueba)"
                )
            }
        };
        RsaKeyPair::from_pem(&private_key_pem, &certificate_pem).map_err(|e| {
            anyhow::anyhow!(
                "error al leer certificados: {e}\n  Verificar que --private-key sea PEM (PKCS#1 o PKCS#8) y --certificate sea X.509 PEM"
            )
        })
    }

    pub fn sign_xml(&self, xml_content: &str, key_pair: &RsaKeyPair) -> anyhow::Result<Vec<u8>> {
        let signer = XSigner {
            xml_document: xml_content.to_string(),
        };
        let signed = signer.sign(key_pair)?;
        Ok(signed)
    }
}
