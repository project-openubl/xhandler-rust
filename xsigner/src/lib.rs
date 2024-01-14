use rcgen::generate_simple_self_signed;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{LineEnding, EncodePublicKey, EncodePrivateKey, DecodePrivateKey, DecodePublicKey};
use x509_parser::prelude::X509Certificate;

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error(transparent)]
    Rsa(#[from] rsa::Error),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub struct RsaKeyPair {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RsaKeyPair {
    pub fn generate(size: usize) -> Result<Self, EncryptionError> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, size)?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            private_key,
            public_key,
        })
    }

    pub fn from(private_key: &str, public_key: &str) -> Result<Self, EncryptionError> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(private_key)?;
        let public_key = RsaPublicKey::from_public_key_pem(public_key)?;

        Ok(Self {
            private_key,
            public_key,
        })
    }

    pub fn encoded_private_key(&self) -> Result<String, rsa::pkcs1::Error> {
        let line_ending = LineEnding::default();
        let pem = self.private_key.to_pkcs8_pem(line_ending)?.to_string();
        Ok(pem)
    }

    pub fn encoded_public_key(&self) -> Result<String, rsa::pkcs1::Error> {
        let line_ending = LineEnding::default();
        let pem = self.public_key.to_public_key_pem(line_ending)?;
        Ok(pem)
    }

    // pub fn a() {
    //     let subject_alt_names :&[_] = &["hello.world.example".to_string(), "localhost".to_string()];
    //     let a = generate_simple_self_signed(subject_alt_names)?;
    // }
}