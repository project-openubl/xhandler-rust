use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use der::{DecodePem, EncodePem};
use libxml::parser::{Parser, XmlParseError};
use libxml::tree::{Document, Node};
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
use rsa::pkcs8::LineEnding;
use rsa::RsaPrivateKey;
use x509_cert::Certificate;
use xml_c14n::{canonicalize_xml, CanonicalizationMode, CanonicalizationOptions};

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
    #[error("Error while signing")]
    Generic,
    #[error("Error `{0}`")]
    GenericWithMessage(String),
    #[error("Error")]
    Std(Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Pkcs1(#[from] rsa::pkcs1::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<()> for SignErr {
    fn from(_error: ()) -> Self {
        Self::Generic
    }
}

impl From<String> for SignErr {
    fn from(error: String) -> Self {
        Self::GenericWithMessage(error)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for SignErr {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Std(error)
    }
}

pub struct XSigner {
    pub xml_document: Document,
}

impl XSigner {
    pub fn from_file(filename: &str) -> Result<Self, XmlParseError> {
        let xml_parser = libxml::parser::Parser::default();
        let xml_document = xml_parser.parse_file(filename)?;
        Ok(Self { xml_document })
    }

    pub fn from_string(xml: &str) -> Result<Self, XmlParseError> {
        let xml_parser = libxml::parser::Parser::default();
        let xml_document = xml_parser.parse_string(xml)?;
        Ok(Self { xml_document })
    }

    pub fn sign(&self, key_pair: &RsaKeyPair) -> Result<(), SignErr> {
        let xml = &self.xml_document;
        let xml_string = xml.to_string();

        let canonicalize_options = CanonicalizationOptions {
            mode: CanonicalizationMode::Canonical1_1,
            keep_comments: false,
            inclusive_ns_prefixes: vec![],
        };
        let xml_canonicalize = canonicalize_xml(&xml_string, canonicalize_options.clone())
            .expect("Could not canonicalize xml");

        // Generate digest
        let digest = hash(MessageDigest::sha256(), xml_canonicalize.as_bytes())
            .expect("Digest generation error");
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
            canonicalize_xml(&signed_info_string, canonicalize_options.clone())
                .expect("Could not canonicalize xml");

        // Sign <ds:SignedInfo>
        let pk_pem = key_pair.private_key_to_pem()?;
        let rsa = Rsa::private_key_from_pem(pk_pem.as_bytes()).expect("Failed to parse PK");
        let pkey = PKey::from_rsa(rsa).expect("Failed to convert RSA to PKey");

        let certificate_pem = key_pair.certificate_to_pem()?;
        let pem_contents = certificate_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut signer =
            Signer::new(MessageDigest::sha256(), &pkey).expect("Signer creation error");
        signer
            .update(signed_info_canonicalize.as_bytes())
            .expect("Failed to update signer");
        let signature = signer.sign_to_vec().expect("Error while signing");
        let signature_base64 = general_purpose::STANDARD.encode(&signature);

        // Search Signature element
        fn find_extension_content_node(node: Node) -> Option<Node> {
            if let Some(ns) = node.get_namespace() {
                if ns.get_prefix() == "ext" && node.get_name() == "ExtensionContent" {
                    return Some(node);
                }
            }

            for child in node.get_child_nodes().into_iter() {
                let result = find_extension_content_node(child);
                if result.is_some() {
                    return result;
                }
            }

            None
        }

        let xml_root_node = xml
            .get_root_element()
            .ok_or(SignErr::Any(anyhow!("Could not get the xml root element")))?;
        let mut extension_content_node = find_extension_content_node(xml_root_node).ok_or(
            SignErr::Any(anyhow!("Could not find the ext:ExtensionContent tag")),
        )?;

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

        let parser = Parser::default();
        let signature_string_node = parser
            .parse_string(&signature_string)
            .expect("Could not parse Signature");
        let mut signed_info_node_root = signature_string_node
            .get_root_element()
            .expect("Could not get root element of Signature");
        signed_info_node_root.unlink();

        extension_content_node.add_child(&mut signed_info_node_root)?;
        Ok(())
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
