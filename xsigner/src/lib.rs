use base64::engine::general_purpose;
use base64::Engine;
use der::{DecodePem, EncodePem};
use openssl::error::ErrorStack;
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
use rsa::pkcs8::LineEnding;
use rsa::RsaPrivateKey;
use std::io::Cursor;
use std::{fs, io};
use x509_cert::Certificate;
use xml_c14n::{
    canonicalize_xml, CanonicalizationErrorCode, CanonicalizationMode, CanonicalizationOptions,
};

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

#[derive(Debug, thiserror::Error)]
pub enum SignErr {
    #[error(transparent)]
    Pkcs1(#[from] rsa::pkcs1::Error),
    #[error(transparent)]
    Key(#[from] ErrorStack),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Canonicalization(#[from] CanonicalizationErrorCode),
}

pub struct XSigner {
    pub xml_document: String,
}

impl XSigner {
    pub fn from_file(filename: &str) -> Result<Self, io::Error> {
        let xml_document = fs::read_to_string(filename)?;
        Ok(Self { xml_document })
    }

    pub fn sign(&self, key_pair: &RsaKeyPair) -> Result<Vec<u8>, SignErr> {
        let canonicalize_options = CanonicalizationOptions {
            mode: CanonicalizationMode::Canonical1_1,
            keep_comments: true,
            inclusive_ns_prefixes: vec![],
        };
        let xml_canonicalize = canonicalize_xml(&self.xml_document, canonicalize_options.clone())?;

        // Generate digest
        let digest = hash(MessageDigest::sha256(), xml_canonicalize.as_bytes())?;
        let digest_base64 = general_purpose::STANDARD.encode(digest);

        // Sign
        let signed_info_string = format!(
            "<ds:SignedInfo xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\">
                <ds:CanonicalizationMethod Algorithm=\"http://www.w3.org/2001/10/xml-exc-c14n#\"/>
                <ds:SignatureMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256\"/>
                <ds:Reference URI=\"\">
                    <ds:Transforms>
                        <ds:Transform Algorithm=\"http://www.w3.org/2000/09/xmldsig#enveloped-signature\"/>
                    </ds:Transforms>
                    <ds:DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmlenc#sha256\"/>
                    <ds:DigestValue>{digest_base64}</ds:DigestValue>
                </ds:Reference>
            </ds:SignedInfo>"
        );
        let signed_info_canonicalize =
            canonicalize_xml(&signed_info_string, canonicalize_options.clone())?;

        // Sign <ds:SignedInfo>
        let pk_pem = key_pair.private_key_to_pem()?;
        let rsa = Rsa::private_key_from_pem(pk_pem.as_bytes())?;
        let pkey = PKey::from_rsa(rsa)?;

        let certificate_pem = key_pair.certificate_to_pem()?;
        let pem_contents = certificate_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
        signer.update(signed_info_canonicalize.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        let signature_base64 = general_purpose::STANDARD.encode(&signature);

        // Signature
        let signature_string = format!(
            "<ds:Signature xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Id=\"PROJECT-OPENUBL\">
                {signed_info_string}
                <ds:SignatureValue>{signature_base64}</ds:SignatureValue>
                <ds:KeyInfo>
                    <ds:X509Data>
                        <ds:X509Certificate>{pem_contents}</ds:X509Certificate>
                    </ds:X509Data>
                </ds:KeyInfo>
            </ds:Signature>"
        );

        let mut xml_reader = quick_xml::Reader::from_str(&xml_canonicalize);
        let mut xml_writer = quick_xml::Writer::new(Cursor::new(Vec::new()));

        let mut inside_target_element = false;
        let mut requires_closing_extension_content_tag = false;

        loop {
            match xml_reader.read_event() {
                Ok(Event::Empty(e)) => {
                    if e.name().as_ref() == b"ext:ExtensionContent" {
                        inside_target_element = true;
                        requires_closing_extension_content_tag = true;

                        xml_writer
                            .write_event(Event::Start(BytesStart::new("ext:ExtensionContent")))?;
                    } else {
                        assert!(xml_writer.write_event(Event::Start(e.clone())).is_ok());
                    }
                }
                Ok(Event::Start(e)) => {
                    assert!(xml_writer.write_event(Event::Start(e.clone())).is_ok());
                    if e.name().as_ref() == b"ext:ExtensionContent" {
                        inside_target_element = true;
                    }
                }
                Ok(Event::End(e)) => {
                    if inside_target_element {
                        inside_target_element = false;

                        let mut xml_content_reader = quick_xml::Reader::from_str(&signature_string);
                        loop {
                            match xml_content_reader.read_event() {
                                Ok(Event::Eof) => break,
                                Ok(e) => assert!(xml_writer.write_event(e).is_ok()),
                                Err(e) => panic!(
                                    "Error at position {}: {:?}",
                                    xml_reader.error_position(),
                                    e
                                ),
                            }
                        }

                        if requires_closing_extension_content_tag {
                            xml_writer
                                .write_event(Event::End(BytesEnd::new("ext:ExtensionContent")))?;
                        }
                    }
                    assert!(xml_writer.write_event(Event::End(e.clone())).is_ok());
                }
                Ok(Event::Eof) => break,
                Ok(e) => assert!(xml_writer.write_event(e).is_ok()),
                Err(e) => panic!("Error at position {}: {:?}", xml_reader.error_position(), e),
            }
        }

        let result = xml_writer.into_inner().into_inner();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::RsaKeyPair;
    use crate::XSigner;

    const RESOURCES: &str = "resources/test";

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

    #[test]
    fn sign_file() {
        let private_key_from_file =
            fs::read_to_string(format!("{RESOURCES}/private.key")).expect("Could not read file");
        let certificate_from_file =
            fs::read_to_string(format!("{RESOURCES}/public.cer")).expect("Could not read file");

        let xml_no_template = format!("{RESOURCES}/invoice_no_template.xml");

        let rsa_key_pair = RsaKeyPair::from_pkcs1_pem_and_certificate(
            &private_key_from_file,
            &certificate_from_file,
        )
        .expect("Could not read certificates");

        let document1 =
            XSigner::from_file(&xml_no_template).expect("Could read xml with no template");

        document1
            .sign(&rsa_key_pair)
            .expect("Could not sign document with no template");
    }
}
