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
    #[arg(short = 'f', long)]
    pub file: String,

    /// Ruta del archivo XML firmado. Si se omite, se imprime en stdout
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    #[command(flatten)]
    pub keys: KeyArgs,

    /// Usar certificados de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,
}

impl SignArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        // Accepts UTF-8 and ISO-8859-1 input
        let signer = if self.file == "-" {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            XSigner::from_bytes(buf)
        } else {
            XSigner::from_path(&self.file)
        }
        .with_context(|| format!("no se puede leer el archivo XML: {}", self.file))?;

        let key_pair = self.keys.resolve(self.beta)?;
        let signed_xml = signer.sign(&key_pair)?;

        match &self.output {
            Some(path) => std::fs::write(path, &signed_xml)?,
            None => {
                let output = String::from_utf8(signed_xml)?;
                print!("{output}");
            }
        }

        Ok(ExitCode::SUCCESS)
    }
}

/// Certificate options shared by `sign` and `apply`
#[derive(Args)]
pub struct KeyArgs {
    /// Ruta al archivo de llave privada PEM, PKCS#1 o PKCS#8 (con --beta se usan certificados de prueba)
    #[arg(long, env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: Option<String>,

    /// Ruta al archivo de certificado X.509 PEM (con --beta se usan certificados de prueba)
    #[arg(long, env = "OPENUBL_CERTIFICATE")]
    pub certificate: Option<String>,

    /// Ruta al certificado PKCS#12 (.pfx/.p12). Tiene prioridad sobre --private-key/--certificate
    #[arg(long, env = "OPENUBL_PFX")]
    pub pfx: Option<String>,

    /// Clave del certificado PKCS#12
    #[arg(long, env = "OPENUBL_PFX_PASSWORD")]
    pub pfx_password: Option<String>,
}

impl KeyArgs {
    pub fn resolve(&self, beta: bool) -> anyhow::Result<RsaKeyPair> {
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
            None if beta => BETA_PRIVATE_KEY.to_string(),
            None => {
                anyhow::bail!(
                    "se requiere --private-key o --pfx (o usar --beta para certificados de prueba)"
                )
            }
        };
        let certificate_pem = match &self.certificate {
            Some(path) => std::fs::read_to_string(path)
                .with_context(|| format!("no se puede leer el certificado: {path}"))?,
            None if beta => BETA_CERTIFICATE.to_string(),
            None => {
                anyhow::bail!(
                    "se requiere --certificate o --pfx (o usar --beta para certificados de prueba)"
                )
            }
        };
        RsaKeyPair::from_pem(&private_key_pem, &certificate_pem).map_err(|e| {
            anyhow::anyhow!(
                "error al leer certificados: {e}\n  Verificar que --private-key sea PEM (PKCS#1 o PKCS#8) y --certificate sea X.509 PEM"
            )
        })
    }
}
