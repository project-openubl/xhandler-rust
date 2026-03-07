use std::io::Read;
use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

#[derive(Args)]
pub struct SignArgs {
    /// Input unsigned XML file. Use "-" for stdin.
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Output signed XML file path. Writes to stdout if omitted.
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Path to PKCS#1 PEM private key file
    #[arg(long = "private-key", env = "OPENUBL_PRIVATE_KEY")]
    pub private_key: String,

    /// Path to X.509 PEM certificate file
    #[arg(long = "certificate", env = "OPENUBL_CERTIFICATE")]
    pub certificate: String,
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

        let private_key_pem = std::fs::read_to_string(&self.private_key)?;
        let certificate_pem = std::fs::read_to_string(&self.certificate)?;

        let key_pair =
            RsaKeyPair::from_pkcs1_pem_and_certificate(&private_key_pem, &certificate_pem)?;

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

    pub fn sign_xml(&self, xml_content: &str, key_pair: &RsaKeyPair) -> anyhow::Result<Vec<u8>> {
        let signer = XSigner {
            xml_document: xml_content.to_string(),
        };
        let signed = signer.sign(key_pair)?;
        Ok(signed)
    }
}
