// use std::str::FromStr;
// use std::time::Duration;

use anyhow::anyhow;
use der::{Decode, DecodePem, Encode, EncodePem};
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
};
use rsa::{RsaPrivateKey, RsaPublicKey};
// use sha2::Sha256;
// use spki::SubjectPublicKeyInfoOwned;
use x509_cert::builder::{Builder, CertificateBuilder, Profile};
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::time::Validity;
use x509_cert::Certificate;

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error(transparent)]
    Rsa(#[from] rsa::Error),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub struct RsaKeyPair {
    private_key: RsaPrivateKey,
    certificate: Certificate,
}

impl RsaKeyPair {
//     pub fn generate(size: usize) -> Result<Self, EncryptionError> {
//         let mut rng = rand::thread_rng();
//         let line_ending = LineEnding::default();

//         let private_key = RsaPrivateKey::new(&mut rng, size)?;
//         let public_key = RsaPublicKey::from(&private_key);
//         let public_key_pem = public_key.to_public_key_pem(line_ending).unwrap();

//         // Generate Certificate
//         let serial_number = SerialNumber::from(42u32);
//         let validity = Validity::from_now(Duration::from_secs(60 * 60 * 24 * 365))
//             .map_err(|_error| anyhow!("Could not generate validity for certificate"))?;
//         let profile = Profile::Root;
//         let subject = Name::from_str("CN=OpenUBL,O=OpenUBL SAC,C=PE")
//             .map_err(|_error| anyhow!("Could not certificate subject from str"))?
//             .to_der()
//             .map_err(|_error| anyhow!("Could not generate validity for certificate"))?;
//         let subject = Name::from_der(&subject)
//             .map_err(|_error| anyhow!("Could not generate validity for certificate from der"))?;
//         let pub_key: spki::SubjectPublicKeyInfo<der::Any, der::asn1::BitString> =
//             SubjectPublicKeyInfoOwned::from_pem(&public_key_pem)
//                 .map_err(|_error| anyhow!("Could not read SubjectPublicKeyInfoOwned"))?;

//         let signer = SigningKey::<Sha256>::new(private_key.clone());

//         let certificate_builder =
//             CertificateBuilder::new(profile, serial_number, validity, subject, pub_key, &signer)
//                 .map_err(|_error| anyhow!("Could not create certificate builder"))?;
//         let certificate = certificate_builder
//             .build()
//             .map_err(|_error| anyhow!("Could not build certificate"))?;

//         Ok(Self {
//             private_key,
//             certificate,
//         })
//     }

    pub fn from_pkcs1_pem_and_certificate(
        private_key: &str,
        certificate: &str,
    ) -> Result<Self, rsa::pkcs8::spki::Error> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(private_key)?;
        let certificate = Certificate::from_pem(certificate)?;

        Ok(Self {
            private_key,
            certificate,
        })
    }

    pub fn private_key_to_pem(&self) -> Result<String, rsa::pkcs1::Error> {
        let line_ending = LineEnding::default();
        let pem = self.private_key.to_pkcs1_pem(line_ending)?.to_string();
        Ok(pem)
    }

    pub fn certificate_to_pem(&self) -> Result<String, rsa::pkcs1::Error> {
        let line_ending = LineEnding::default();
        let pem = self.certificate.to_pem(line_ending)?;
        Ok(pem)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::RsaKeyPair;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn generate() {
        // let keypair = RsaKeyPair::generate(256).unwrap();
        // let private = keypair.private_key_to_pem().unwrap();
        // let certificate = keypair.certificate_to_pem().unwrap();

        // let _keypair = RsaKeyPair::from(&private, &certificate).unwrap();
    }

    #[test]
    fn read_from_file() {
        let private_key_from_file =
            fs::read_to_string(format!("{RESOURCES}/private.key")).expect("Could not read file");
        let certificate_from_file =
            fs::read_to_string(format!("{RESOURCES}/public.cer")).expect("Could not read file");

        let keypair = RsaKeyPair::from_pkcs1_pem_and_certificate(
            &private_key_from_file,
            &certificate_from_file,
        )
        .unwrap();
        let private = keypair.private_key_to_pem().unwrap();
        let certificate = keypair.certificate_to_pem().unwrap();

        assert_eq!(private_key_from_file, private);
        assert_eq!(certificate_from_file, certificate);
    }
}
