use anyhow::anyhow;
use der::{DecodePem, EncodePem};
use libxml::parser::XmlParseError;
use libxml::tree::{Document, Namespace, Node};
use libxml::xpath::Context;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
use rsa::pkcs8::LineEnding;
use rsa::RsaPrivateKey;
use x509_cert::Certificate;
use xmlsec::{XmlSecError, XmlSecKey, XmlSecKeyFormat, XmlSecSignatureContext};

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
    XmlSec(#[from] XmlSecError),
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

    pub fn sign(&self, key_pair: &RsaKeyPair) -> Result<(), SignErr> {
        let xml = &self.xml_document;

        // Search Signature element
        let context = Context::new(xml)?;
        let signature_node = context.evaluate("//ds:Signature");

        // Add the Signature xml tag
        if signature_node.is_err() {
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

            let root = xml
                .get_root_element()
                .ok_or(SignErr::Any(anyhow!("Could not get the xml root element")))?;
            let mut root_note = find_extension_content_node(root).ok_or(SignErr::Any(anyhow!(
                "Could not find the ext:ExtensionContent tag"
            )))?;

            // Signature
            let mut signature = Node::new("Signature", None, xml)?;
            signature.set_attribute("Id", "PROJECT-OPENUBL")?;
            let ns = Namespace::new("ds", "http://www.w3.org/2000/09/xmldsig#", &mut signature)?;
            signature.set_namespace(&ns)?;

            //
            let mut signed_info = Node::new("SignedInfo", Some(ns.clone()), xml)?;
            signature.add_child(&mut signed_info)?;

            let mut canonicalization_method =
                Node::new("CanonicalizationMethod", Some(ns.clone()), xml)?;
            canonicalization_method.set_attribute(
                "Algorithm",
                "http://www.w3.org/TR/2001/REC-xml-c14n-20010315",
            )?;
            signed_info.add_child(&mut canonicalization_method)?;

            let mut signature_method = Node::new("SignatureMethod", Some(ns.clone()), xml)?;
            signature_method
                .set_attribute("Algorithm", "http://www.w3.org/2000/09/xmldsig#rsa-sha1")?;
            signed_info.add_child(&mut signature_method)?;

            let mut reference = Node::new("Reference", Some(ns.clone()), xml)?;
            reference.set_attribute("URI", "")?;
            signed_info.add_child(&mut reference)?;

            let mut transforms = Node::new("Transforms", Some(ns.clone()), xml)?;
            reference.add_child(&mut transforms)?;

            let mut transform = Node::new("Transform", Some(ns.clone()), xml)?;
            transform.set_attribute(
                "Algorithm",
                "http://www.w3.org/2000/09/xmldsig#enveloped-signature",
            )?;
            transforms.add_child(&mut transform)?;

            let mut digest_method = Node::new("DigestMethod", Some(ns.clone()), xml)?;
            digest_method.set_attribute("Algorithm", "http://www.w3.org/2000/09/xmldsig#sha1")?;
            reference.add_child(&mut digest_method)?;

            let mut digest_value = Node::new("DigestValue", Some(ns.clone()), xml)?;
            reference.add_child(&mut digest_value)?;

            let mut signature_value = Node::new("SignatureValue", Some(ns.clone()), xml)?;
            signature.add_child(&mut signature_value)?;

            let mut key_info = Node::new("KeyInfo", Some(ns.clone()), xml)?;
            signature.add_child(&mut key_info)?;

            let mut x509_data = Node::new("X509Data", Some(ns.clone()), xml)?;
            key_info.add_child(&mut x509_data)?;

            let mut x509_certificate = Node::new("X509Certificate", Some(ns.clone()), xml)?;
            x509_data.add_child(&mut x509_certificate)?;

            //
            root_note.add_child(&mut signature)?;
        }

        let private_key_pem = key_pair.private_key_to_pem()?;
        let private_key =
            XmlSecKey::from_memory(private_key_pem.as_bytes(), XmlSecKeyFormat::Pem, None)?;

        let certificate_pem = key_pair.certificate_to_pem()?;
        private_key.load_cert_from_memory(certificate_pem.as_bytes(), XmlSecKeyFormat::CertPem)?;

        let mut sigctx = XmlSecSignatureContext::new();
        sigctx.insert_key(private_key);

        sigctx.sign_document(xml).expect("Failed to sign document");

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
        let xml_with_template = format!("{RESOURCES}/invoice_with_template.xml");

        let rsa_key_pair = RsaKeyPair::from_pkcs1_pem_and_certificate(
            &private_key_from_file,
            &certificate_from_file,
        )
        .expect("Could not read certificates");

        let document1 =
            XSigner::from_file(&xml_no_template).expect("Could read xml with no template");
        let document2 =
            XSigner::from_file(&xml_with_template).expect("Could read xml with template");

        document1
            .sign(&rsa_key_pair)
            .expect("Could not sign document with no tempate");
        document2
            .sign(&rsa_key_pair)
            .expect("Could not sign document with template");
    }
}
