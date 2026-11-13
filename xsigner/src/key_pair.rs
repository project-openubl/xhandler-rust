use p12_keystore::{KeyStore, Pkcs12ImportPolicy};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::signature::{SignatureEncoding, Signer};
use rsa::{PrivateKey, PublicKey};
use sha2::Sha256;
use x509_cert::der::pem::LineEnding;
use x509_cert::der::{Decode, DecodePem, Encode, EncodePem};
use x509_cert::Certificate;

use crate::SignError;

/// Private key and X.509 certificate used to sign
pub struct RsaKeyPair {
    signing_key: SigningKey<Sha256>,
    certificate: Certificate,
    certificate_der: Vec<u8>,
}

impl RsaKeyPair {
    /// Private key in PEM (PKCS#1 `RSA PRIVATE KEY` or PKCS#8 `PRIVATE KEY`)
    /// and X.509 certificate in PEM.
    pub fn from_pem(private_key: &str, certificate: &str) -> Result<Self, SignError> {
        let private_key = if private_key.contains("-----BEGIN RSA PRIVATE KEY-----") {
            PrivateKey::from_pkcs1_pem(private_key).map_err(key_error)?
        } else {
            PrivateKey::from_pkcs8_pem(private_key).map_err(key_error)?
        };
        let certificate = Certificate::from_pem(certificate).map_err(key_error)?;
        Self::new(private_key, certificate)
    }

    /// PKCS#12 keystore (`.pfx` / `.p12`), the format SUNAT certificates are
    /// usually delivered in. Legacy RC2/3DES keystores are supported. The key
    /// and its certificate are paired by their `localKeyId`.
    pub fn from_pkcs12(der: &[u8], password: &str) -> Result<Self, SignError> {
        let keystore =
            KeyStore::from_pkcs12(der, password, Pkcs12ImportPolicy::Strict).map_err(|error| {
                SignError::Key(format!(
                    "Could not open the PKCS#12 keystore, check its password ({error})"
                ))
            })?;
        let missing = || {
            SignError::Key(
                "The PKCS#12 keystore must contain a private key and its certificate".into(),
            )
        };
        let (_, chain) = keystore.private_key_chain().ok_or_else(missing)?;
        let certificate = chain.certs().first().ok_or_else(missing)?;

        let private_key = PrivateKey::from_pkcs8_der(chain.key().as_der()).map_err(key_error)?;
        let certificate = Certificate::from_der(certificate.as_der()).map_err(key_error)?;
        Self::new(private_key, certificate)
    }

    fn new(private_key: PrivateKey, certificate: Certificate) -> Result<Self, SignError> {
        let public_key_info = certificate
            .tbs_certificate()
            .subject_public_key_info()
            .to_der()
            .map_err(key_error)?;
        let certificate_key =
            PublicKey::from_public_key_der(&public_key_info).map_err(key_error)?;
        if certificate_key != private_key.to_public_key() {
            return Err(SignError::Key(
                "The certificate does not belong to the private key".into(),
            ));
        }

        Ok(Self {
            signing_key: SigningKey::new(private_key),
            certificate_der: certificate.to_der().map_err(key_error)?,
            certificate,
        })
    }

    /// X.509 certificate in DER, e.g. to read its subject or validity
    pub fn certificate_der(&self) -> &[u8] {
        &self.certificate_der
    }

    pub fn certificate_to_pem(&self) -> Result<String, SignError> {
        self.certificate.to_pem(LineEnding::LF).map_err(key_error)
    }

    /// RSA PKCS#1 v1.5 signature with SHA-256
    pub(crate) fn sign(&self, data: &[u8]) -> Result<Vec<u8>, SignError> {
        let signature = self.signing_key.try_sign(data).map_err(key_error)?;
        Ok(signature.to_vec())
    }
}

fn key_error(error: impl std::fmt::Display) -> SignError {
    SignError::Key(format!("Invalid private key or certificate: {error}"))
}
