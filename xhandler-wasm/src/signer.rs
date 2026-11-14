use serde::Deserialize;
use serde_json::Value;
use xhandler::xsigner::{RsaKeyPair, SignError, XSigner};

use crate::bytes::Base64;
use crate::ops::{Error, ErrorKind, OpResult};

#[derive(Deserialize)]
struct Input {
    /// UTF-8 or ISO-8859-1 XML
    xml: Base64,
    key: Key,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Key {
    Pem {
        private_key: String,
        certificate: String,
    },
    Pkcs12 {
        der: Base64,
        password: String,
    },
}

/// `{xml, key: {pem: {private_key, certificate}} | {pkcs12: {der, password}}}`
/// -> the signed XML (UTF-8), in base64.
pub fn sign(input: Value) -> OpResult {
    // serde quotes the values it rejects, which here may be key material
    let Input { xml, key } = serde_json::from_value(input).map_err(|_| {
        Error::new(
            ErrorKind::InvalidInput,
            anyhow::anyhow!("expected {{xml: base64, key: {{pem}} | {{pkcs12}}}}"),
        )
    })?;
    let key_pair = match key {
        Key::Pem {
            private_key,
            certificate,
        } => RsaKeyPair::from_pem(&private_key, &certificate),
        Key::Pkcs12 { der, password } => RsaKeyPair::from_pkcs12(&der.0, &password),
    }
    .map_err(sign_error)?;

    let signer = XSigner::from_bytes(xml.0).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    let signed = signer.sign(&key_pair).map_err(sign_error)?;
    Ok(Base64(signed).into())
}

fn sign_error(error: SignError) -> Error {
    let kind = match error {
        SignError::Key(_) | SignError::WrongPassword => ErrorKind::InvalidKey,
        SignError::QuickXml(_) | SignError::Invalid(_) => ErrorKind::InvalidInput,
    };
    Error::new(kind, error)
}
