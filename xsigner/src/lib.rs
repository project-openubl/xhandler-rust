use std::path::Path;
use std::{fs, io};

use base64::engine::general_purpose;
use base64::Engine;
use quick_xml::events::Event;
use quick_xml::name::ResolveResult;
use quick_xml::NsReader;
use sha2::{Digest, Sha256};

mod c14n;
mod key_pair;

pub use key_pair::RsaKeyPair;

/// `Id` of the `ds:Signature`, referenced by the `cac:Signature` of the
/// xbuilder templates (`#PROJECT-OPENUBL-SIGN`).
pub const SIGNATURE_ID: &str = "PROJECT-OPENUBL-SIGN";

const DS_NS: &str = "http://www.w3.org/2000/09/xmldsig#";
const EXT_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2";

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error(transparent)]
    QuickXml(#[from] quick_xml::Error),
    #[error("{0}")]
    Key(String),
    #[error("{0}")]
    Invalid(String),
}

pub struct XSigner {
    pub xml_document: String,
}

impl XSigner {
    /// Reads UTF-8 or ISO-8859-1 XML.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, io::Error> {
        let xml_document = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(error) if declares_iso_8859_1(error.as_bytes()) => {
                error.as_bytes().iter().map(|&byte| byte as char).collect()
            }
            Err(error) => return Err(io::Error::new(io::ErrorKind::InvalidData, error)),
        };
        Ok(Self { xml_document })
    }

    /// Reads a UTF-8 or ISO-8859-1 XML file.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        Self::from_bytes(fs::read(path)?)
    }

    /// Signs the document with an enveloped RSA-SHA256 signature placed in the
    /// first empty `ext:ExtensionContent` (created if the document has none).
    ///
    /// Returns the original document plus the signature, as UTF-8 with a UTF-8
    /// XML declaration. The rest of the document is left untouched.
    pub fn sign(&self, key_pair: &RsaKeyPair) -> Result<Vec<u8>, SignError> {
        // The text is already decoded: without its declaration (which may say
        // e.g. encoding="ISO-8859-1") it is parsed as what it is, UTF-8.
        let mut document = strip_xml_declaration(&self.xml_document).to_string();
        let position = prepare_signature_slot(&mut document)?;

        // Reference URI="" + enveloped transform: the digest is the inclusive
        // C14N 1.0 (without comments) of the document without the signature.
        let canonical_document = c14n::canonicalize(&document)?;
        let digest_base64 =
            general_purpose::STANDARD.encode(Sha256::digest(canonical_document.as_bytes()));

        let signed_info = format!(
            "<ds:SignedInfo xmlns:ds=\"{DS_NS}\">\
<ds:CanonicalizationMethod Algorithm=\"http://www.w3.org/2001/10/xml-exc-c14n#\"/>\
<ds:SignatureMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256\"/>\
<ds:Reference URI=\"\">\
<ds:Transforms><ds:Transform Algorithm=\"http://www.w3.org/2000/09/xmldsig#enveloped-signature\"/></ds:Transforms>\
<ds:DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmlenc#sha256\"/>\
<ds:DigestValue>{digest_base64}</ds:DigestValue>\
</ds:Reference>\
</ds:SignedInfo>"
        );
        // Exclusive C14N, as declared. SignedInfo only uses the ds namespace,
        // declared on itself, so its exclusive and inclusive canonical forms
        // are the same and don't depend on where it is inserted.
        let canonical_signed_info = c14n::canonicalize(&signed_info)?;

        let signature_value =
            general_purpose::STANDARD.encode(key_pair.sign(canonical_signed_info.as_bytes())?);
        let certificate = general_purpose::STANDARD.encode(key_pair.certificate_der());

        let signature = format!(
            "<ds:Signature xmlns:ds=\"{DS_NS}\" Id=\"{SIGNATURE_ID}\">\
{signed_info}\
<ds:SignatureValue>{signature_value}</ds:SignatureValue>\
<ds:KeyInfo><ds:X509Data><ds:X509Certificate>{certificate}</ds:X509Certificate></ds:X509Data></ds:KeyInfo>\
</ds:Signature>"
        );
        document.insert_str(position, &signature);

        Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{document}").into_bytes())
    }
}

/// Returns the byte offset where the signature goes: inside the first
/// `ext:ExtensionContent` without child elements. A self-closing
/// `<ext:ExtensionContent/>` is expanded first, and `ext:UBLExtensions` is added
/// when the document has none; neither changes the document's C14N form.
fn prepare_signature_slot(document: &mut String) -> Result<usize, SignError> {
    match find_empty_extension_content(document)? {
        Some(Slot::Inside(position)) => Ok(position),
        Some(Slot::SelfClosing { start, end, name }) => {
            let start_tag = format!(
                "{}>",
                document[start..end].trim_end_matches("/>").trim_end()
            );
            document.replace_range(start..end, &format!("{start_tag}</{name}>"));
            Ok(start + start_tag.len())
        }
        None => add_ubl_extensions(document),
    }
}

/// An empty `ext:ExtensionContent`, as byte offsets into the document text
enum Slot {
    /// Right after an `<ext:ExtensionContent>` start tag with no child elements
    Inside(usize),
    /// A self-closing `<ext:ExtensionContent/>` tag
    SelfClosing {
        start: usize,
        end: usize,
        name: String,
    },
}

fn is_extension_content(namespace: &ResolveResult, local_name: &[u8]) -> bool {
    matches!(namespace, ResolveResult::Bound(ns) if ns.as_ref() == EXT_NS.as_bytes())
        && local_name == b"ExtensionContent"
}

/// Finds the first `ext:ExtensionContent` without child elements. Returns
/// `None` when the document has no `ext:ExtensionContent` at all.
fn find_empty_extension_content(document: &str) -> Result<Option<Slot>, SignError> {
    let mut reader = NsReader::from_str(document);
    // Byte offset right after an open ExtensionContent that has no children yet
    let mut open_content: Option<usize> = None;
    let mut has_extension_content = false;

    loop {
        let start = reader.buffer_position() as usize;
        let (namespace, event) = reader.read_resolved_event()?;
        match event {
            Event::Empty(e) if is_extension_content(&namespace, e.local_name().as_ref()) => {
                return Ok(Some(Slot::SelfClosing {
                    start,
                    end: reader.buffer_position() as usize,
                    name: String::from_utf8_lossy(e.name().as_ref()).into_owned(),
                }));
            }
            Event::Start(e) if is_extension_content(&namespace, e.local_name().as_ref()) => {
                has_extension_content = true;
                open_content = Some(reader.buffer_position() as usize);
            }
            Event::End(e) if is_extension_content(&namespace, e.local_name().as_ref()) => {
                if let Some(position) = open_content {
                    return Ok(Some(Slot::Inside(position)));
                }
            }
            // Any child element means this ExtensionContent is already in use
            Event::Start(_) | Event::Empty(_) => open_content = None,
            Event::Eof if has_extension_content => {
                return Err(SignError::Invalid(
                    "Every ext:ExtensionContent already has content; add an empty one for the signature".into(),
                ))
            }
            Event::Eof => return Ok(None),
            _ => {}
        }
    }
}

/// Inserts an empty `ext:UBLExtensions` block as the first child of the root
/// and returns the offset inside its `ext:ExtensionContent`.
fn add_ubl_extensions(document: &mut String) -> Result<usize, SignError> {
    let mut reader = NsReader::from_str(document);
    loop {
        match reader.read_event()? {
            Event::Start(_) => break,
            Event::Eof | Event::Empty(_) => {
                return Err(SignError::Invalid("The XML has no root element".into()))
            }
            _ => {}
        }
    }

    let position = reader.buffer_position() as usize;
    let extensions = format!(
        "<ext:UBLExtensions xmlns:ext=\"{EXT_NS}\"><ext:UBLExtension><ext:ExtensionContent></ext:ExtensionContent></ext:UBLExtension></ext:UBLExtensions>"
    );
    let content_offset = extensions
        .find("</ext:ExtensionContent>")
        .unwrap_or_default();
    document.insert_str(position, &extensions);
    Ok(position + content_offset)
}

fn strip_xml_declaration(xml: &str) -> &str {
    let xml = xml.trim_start_matches('\u{feff}').trim_start();
    match xml
        .strip_prefix("<?xml")
        .and_then(|rest| rest.find("?>").map(|end| (rest, end)))
    {
        Some((rest, end)) => rest[end + 2..].trim_start(),
        None => xml,
    }
}

fn declares_iso_8859_1(bytes: &[u8]) -> bool {
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(100)]).to_lowercase();
    head.starts_with("<?xml") && head.contains("iso-8859-1")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use base64::engine::general_purpose;
    use base64::Engine;
    use rsa::pkcs1::DecodeRsaPrivateKey;
    use rsa::pkcs1v15::{Signature, VerifyingKey};
    use rsa::pkcs8::{DecodePublicKey, EncodePrivateKey, LineEnding};
    use rsa::signature::Verifier;
    use rsa::{PrivateKey, PublicKey};
    use sha2::{Digest, Sha256};
    use x509_cert::der::{Decode, Encode};
    use x509_cert::Certificate;

    use crate::{c14n, RsaKeyPair, SignError, XSigner, DS_NS, SIGNATURE_ID};

    const RESOURCES: &str = "resources/test";
    const HEAD: &str = r#"<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2" xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2" xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2">"#;

    fn key_pair() -> RsaKeyPair {
        let private_key = fs::read_to_string(format!("{RESOURCES}/private.key")).unwrap();
        let certificate = fs::read_to_string(format!("{RESOURCES}/public.cer")).unwrap();
        RsaKeyPair::from_pem(&private_key, &certificate).expect("Could not read certificates")
    }

    fn sign(xml: &str) -> Result<String, SignError> {
        let signer = XSigner {
            xml_document: xml.to_string(),
        };
        signer
            .sign(&key_pair())
            .map(|signed| String::from_utf8(signed).unwrap())
    }

    fn between<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
        let from = text.find(start).unwrap() + start.len();
        &text[from..from + text[from..].find(end).unwrap()]
    }

    /// Verifies the signature the way a validator (e.g. SUNAT) does: digest of
    /// the document without the ds:Signature, and RSA-SHA256 over SignedInfo.
    fn assert_valid_signature(signed: &str) {
        let base64 = |start, end| {
            general_purpose::STANDARD
                .decode(between(signed, start, end))
                .unwrap()
        };

        let signature_start = signed.find("<ds:Signature ").unwrap();
        let signature_end = signed.find("</ds:Signature>").unwrap() + "</ds:Signature>".len();
        let unsigned = format!("{}{}", &signed[..signature_start], &signed[signature_end..]);
        let canonical = c14n::canonicalize(&unsigned).unwrap();
        assert_eq!(
            Sha256::digest(canonical.as_bytes()).to_vec(),
            base64("<ds:DigestValue>", "</ds:DigestValue>")
        );

        let signed_info_start = format!("<ds:SignedInfo xmlns:ds=\"{DS_NS}\">");
        let signed_info = format!(
            "{signed_info_start}{}</ds:SignedInfo>",
            between(signed, &signed_info_start, "</ds:SignedInfo>")
        );
        let canonical_signed_info = c14n::canonicalize(&signed_info).unwrap();

        let certificate =
            Certificate::from_der(&base64("<ds:X509Certificate>", "</ds:X509Certificate>"))
                .unwrap();
        let public_key_info = certificate.tbs_certificate().subject_public_key_info();
        let public_key =
            PublicKey::from_public_key_der(&public_key_info.to_der().unwrap()).unwrap();
        let signature =
            Signature::try_from(base64("<ds:SignatureValue>", "</ds:SignatureValue>").as_slice())
                .unwrap();
        VerifyingKey::<Sha256>::new(public_key)
            .verify(canonical_signed_info.as_bytes(), &signature)
            .expect("Invalid SignatureValue");
    }

    #[test]
    fn read_pem_keys() {
        let certificate = fs::read_to_string(format!("{RESOURCES}/public.cer")).unwrap();
        assert_eq!(key_pair().certificate_to_pem().unwrap(), certificate);

        // PKCS#8 private keys are accepted too
        let pkcs1 = fs::read_to_string(format!("{RESOURCES}/private.key")).unwrap();
        let pkcs8 = PrivateKey::from_pkcs1_pem(&pkcs1)
            .unwrap()
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap();
        assert!(pkcs8.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(RsaKeyPair::from_pem(&pkcs8, &certificate).is_ok());
    }

    #[test]
    fn read_pkcs12_keystore() {
        let pfx = fs::read(format!(
            "{RESOURCES}/LLAMA-PE-CERTIFICADO-DEMO-10467793549.pfx"
        ))
        .unwrap();
        let key_pair = RsaKeyPair::from_pkcs12(&pfx, "password").expect("Could not read pfx");
        assert!(key_pair.certificate_to_pem().is_ok());
        assert!(RsaKeyPair::from_pkcs12(&pfx, "wrong password").is_err());
    }

    /// Older keystores (e.g. exported from Windows) encrypt with 3DES
    #[test]
    fn read_legacy_pkcs12_keystore() {
        let pfx = fs::read(format!("{RESOURCES}/legacy_3des.pfx")).unwrap();
        let key_pair = RsaKeyPair::from_pkcs12(&pfx, "password").expect("Could not read pfx");
        let certificate = fs::read_to_string(format!("{RESOURCES}/public.cer")).unwrap();
        assert_eq!(key_pair.certificate_to_pem().unwrap(), certificate);

        let signed = XSigner::from_path(format!("{RESOURCES}/invoice_no_template.xml"))
            .unwrap()
            .sign(&key_pair)
            .unwrap();
        assert_valid_signature(&String::from_utf8(signed).unwrap());
    }

    #[test]
    fn sign_file() {
        let document = XSigner::from_path(format!("{RESOURCES}/invoice_no_template.xml"))
            .expect("Could read xml with no template");
        let signed = String::from_utf8(document.sign(&key_pair()).unwrap()).unwrap();

        assert!(signed.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Invoice"));
        assert!(signed.contains(&format!("<ext:ExtensionContent><ds:Signature xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Id=\"{SIGNATURE_ID}\">")));
        assert_eq!(signed.matches("<ds:Signature ").count(), 1);
        assert_valid_signature(&signed);
    }

    /// Each `signed/<name>.signed.xml` was produced from `signed/<name>.xml`
    /// by the previous libxml2/OpenSSL implementation; RSA PKCS#1 v1.5 is
    /// deterministic, so the output must be byte-for-byte the same.
    #[test]
    fn sign_matches_previous_implementation() {
        let mut checked = 0;
        for entry in fs::read_dir(format!("{RESOURCES}/signed")).unwrap() {
            let expected_path = entry.unwrap().path();
            let Some(source) = expected_path.to_str().unwrap().strip_suffix(".signed.xml") else {
                continue;
            };
            let signed = XSigner::from_path(format!("{source}.xml"))
                .unwrap()
                .sign(&key_pair())
                .unwrap();
            assert!(signed == fs::read(&expected_path).unwrap(), "{source}");
            checked += 1;
        }
        assert!(checked > 0);
    }

    #[test]
    fn read_key_pair_rejects_a_certificate_of_another_key() {
        let other_key = fs::read_to_string(format!("{RESOURCES}/other_private.key")).unwrap();
        let certificate = fs::read_to_string(format!("{RESOURCES}/public.cer")).unwrap();
        assert!(matches!(
            RsaKeyPair::from_pem(&other_key, &certificate),
            Err(SignError::Key(_))
        ));
    }

    #[test]
    fn sign_keeps_document_text_and_comments() {
        let xml = format!(
            "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n{HEAD}\n    <ext:UBLExtensions><ext:UBLExtension><ext:ExtensionContent/></ext:UBLExtension></ext:UBLExtensions>\n    <!-- a comment -->\n    <cbc:Note><![CDATA[José Peña Ñandú]]></cbc:Note>\n    <cbc:Empty/>\n</Invoice>"
        );
        let signed = sign(&xml).unwrap();

        assert!(signed.contains("<!-- a comment -->"));
        assert!(signed.contains("<![CDATA[José Peña Ñandú]]>"));
        assert!(signed.contains("<cbc:Empty/>"));
        assert_valid_signature(&signed);
    }

    #[test]
    fn sign_uses_first_empty_extension_content() {
        let xml = format!(
            "{HEAD}<ext:UBLExtensions><ext:UBLExtension><ext:ExtensionContent><cbc:Note>info</cbc:Note></ext:ExtensionContent></ext:UBLExtension><ext:UBLExtension><ext:ExtensionContent>\n</ext:ExtensionContent></ext:UBLExtension></ext:UBLExtensions></Invoice>"
        );
        let signed = sign(&xml).unwrap();

        assert!(signed.contains("<cbc:Note>info</cbc:Note></ext:ExtensionContent>"));
        assert!(signed.contains("<ext:UBLExtension><ext:ExtensionContent><ds:Signature "));
        assert_valid_signature(&signed);
    }

    #[test]
    fn sign_adds_ubl_extensions_when_missing() {
        let signed = sign(&format!("{HEAD}<cbc:ID>F001-1</cbc:ID></Invoice>")).unwrap();

        assert!(signed.contains(&format!("{HEAD}<ext:UBLExtensions ")));
        assert_valid_signature(&signed);
    }

    #[test]
    fn sign_fails_when_extension_contents_are_in_use() {
        let xml = format!(
            "{HEAD}<ext:UBLExtensions><ext:UBLExtension><ext:ExtensionContent><cbc:Note>info</cbc:Note></ext:ExtensionContent></ext:UBLExtension></ext:UBLExtensions></Invoice>"
        );
        assert!(matches!(sign(&xml), Err(SignError::Invalid(_))));
    }
}
