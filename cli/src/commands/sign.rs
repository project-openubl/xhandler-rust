use std::io::Read;
use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

pub const BETA_PRIVATE_KEY: &str = include_str!("../../../xsigner/resources/test/private.key");
pub const BETA_CERTIFICATE: &str = include_str!("../../../xsigner/resources/test/public.cer");

#[derive(Args)]
pub struct SignArgs {
    /// Archivo XML sin firmar. Usar "-" para leer desde stdin
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo XML firmado. Si se omite, se imprime en stdout
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Ruta al archivo de llave privada PKCS#1 PEM (con --beta se usan certificados de prueba)
    #[arg(long = "private-key", env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: Option<String>,

    /// Ruta al archivo de certificado X.509 PEM (con --beta se usan certificados de prueba)
    #[arg(long = "certificate", env = "OPENUBL_CERTIFICATE")]
    pub certificate: Option<String>,

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
            std::fs::read_to_string(&self.input_file)?
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
        let private_key_pem = match &self.private_key {
            Some(path) => std::fs::read_to_string(path)?,
            None if self.beta => BETA_PRIVATE_KEY.to_string(),
            None => {
                anyhow::bail!("--private-key is required (or use --beta for test certificates)")
            }
        };
        let certificate_pem = match &self.certificate {
            Some(path) => std::fs::read_to_string(path)?,
            None if self.beta => BETA_CERTIFICATE.to_string(),
            None => {
                anyhow::bail!("--certificate is required (or use --beta for test certificates)")
            }
        };
        Ok(RsaKeyPair::from_pkcs1_pem_and_certificate(
            &private_key_pem,
            &certificate_pem,
        )?)
    }

    pub fn sign_xml(&self, xml_content: &str, key_pair: &RsaKeyPair) -> anyhow::Result<Vec<u8>> {
        let signer = XSigner {
            xml_document: xml_content.to_string(),
        };
        let signed = signer.sign(key_pair)?;
        Ok(signed)
    }
}
