use std::path::Path;
use std::sync::OnceLock;
use std::{fs, io};

use base64::engine::general_purpose;
use base64::Engine;
use openssl::error::ErrorStack;
use openssl::hash::{hash, MessageDigest};
use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Private};
use openssl::provider::Provider;
use openssl::sign::Signer;
use openssl::x509::X509;
use quick_xml::events::Event;
use quick_xml::name::ResolveResult;
use quick_xml::NsReader;
use xml_c14n::{
    canonicalize_xml, CanonicalizationErrorCode, CanonicalizationMode, CanonicalizationOptions,
};

/// `Id` of the `ds:Signature`, referenced by the `cac:Signature` of the
/// xbuilder templates (`#PROJECT-OPENUBL-SIGN`).
pub const SIGNATURE_ID: &str = "PROJECT-OPENUBL-SIGN";

const DS_NS: &str = "http://www.w3.org/2000/09/xmldsig#";
const EXT_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2";

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error(transparent)]
    OpenSsl(#[from] ErrorStack),
    #[error(transparent)]
    Canonicalization(#[from] CanonicalizationErrorCode),
    #[error(transparent)]
    QuickXml(#[from] quick_xml::Error),
    #[error("{0}")]
    Invalid(String),
}

/// Private key and X.509 certificate used to sign
pub struct RsaKeyPair {
    private_key: PKey<Private>,
    certificate: X509,
}

impl RsaKeyPair {
    /// Private key in PEM (PKCS#1 `RSA PRIVATE KEY` or PKCS#8 `PRIVATE KEY`)
    /// and X.509 certificate in PEM.
    pub fn from_pem(private_key: &str, certificate: &str) -> Result<Self, SignError> {
        Ok(Self {
            private_key: PKey::private_key_from_pem(private_key.as_bytes())?,
            certificate: X509::from_pem(certificate.as_bytes())?,
        })
    }

    /// PKCS#12 keystore (`.pfx` / `.p12`), the format SUNAT certificates are
    /// usually delivered in.
    pub fn from_pkcs12(der: &[u8], password: &str) -> Result<Self, SignError> {
        let pkcs12 = Pkcs12::from_der(der)?;
        let keystore = match pkcs12.parse2(password) {
            Ok(keystore) => keystore,
            Err(error) => {
                // Older keystores (e.g. exported from Windows) use RC2/3DES,
                // which OpenSSL 3 only offers through its legacy provider.
                if !load_legacy_provider() {
                    return Err(error.into());
                }
                pkcs12.parse2(password)?
            }
        };
        match (keystore.pkey, keystore.cert) {
            (Some(private_key), Some(certificate)) => Ok(Self {
                private_key,
                certificate,
            }),
            _ => Err(SignError::Invalid(
                "The PKCS#12 keystore must contain a private key and its certificate".into(),
            )),
        }
    }

    pub fn certificate_to_pem(&self) -> Result<String, ErrorStack> {
        let pem = self.certificate.to_pem()?;
        Ok(String::from_utf8_lossy(&pem).into_owned())
    }
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
        // Without its declaration the text is parsed as what it is: UTF-8.
        // Keeping e.g. encoding="ISO-8859-1" would make libxml2 re-decode it.
        let mut document = strip_xml_declaration(&self.xml_document).to_string();
        let position = prepare_signature_slot(&mut document)?;

        // Reference URI="" + enveloped transform: the digest is the inclusive
        // C14N 1.0 (without comments) of the document without the signature.
        let canonical_document = canonicalize(&document, CanonicalizationMode::Canonical1_0)?;
        let digest = hash(MessageDigest::sha256(), canonical_document.as_bytes())?;
        let digest_base64 = general_purpose::STANDARD.encode(digest);

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
        // Exclusive C14N, as declared: SignedInfo only uses the ds namespace,
        // so its canonical form doesn't depend on where it is inserted.
        let canonical_signed_info =
            canonicalize(&signed_info, CanonicalizationMode::ExclusiveCanonical1_0)?;

        let mut signer = Signer::new(MessageDigest::sha256(), &key_pair.private_key)?;
        signer.update(canonical_signed_info.as_bytes())?;
        let signature_value = general_purpose::STANDARD.encode(signer.sign_to_vec()?);
        let certificate = general_purpose::STANDARD.encode(key_pair.certificate.to_der()?);

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

/// C14N without comments
fn canonicalize(
    xml: &str,
    mode: CanonicalizationMode,
) -> Result<String, CanonicalizationErrorCode> {
    canonicalize_xml(
        xml,
        CanonicalizationOptions {
            mode,
            keep_comments: false,
            inclusive_ns_prefixes: vec![],
        },
    )
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

/// Loads OpenSSL's legacy provider once for the whole process (keeping the
/// default provider). Returns whether it is available.
fn load_legacy_provider() -> bool {
    static LEGACY: OnceLock<Option<Provider>> = OnceLock::new();
    LEGACY
        .get_or_init(|| Provider::try_load(None, "legacy", true).ok())
        .is_some()
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
    use openssl::hash::{hash, MessageDigest};
    use openssl::sign::Verifier;
    use openssl::x509::X509;
    use xml_c14n::CanonicalizationMode;

    use crate::{canonicalize, RsaKeyPair, SignError, XSigner, DS_NS, SIGNATURE_ID};

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
        let unsigned = unsigned.split_once("?>").unwrap().1;
        let canonical = canonicalize(unsigned, CanonicalizationMode::Canonical1_0).unwrap();
        let digest = hash(MessageDigest::sha256(), canonical.as_bytes()).unwrap();
        assert_eq!(
            digest.to_vec(),
            base64("<ds:DigestValue>", "</ds:DigestValue>")
        );

        let signed_info_start = format!("<ds:SignedInfo xmlns:ds=\"{DS_NS}\">");
        let signed_info = format!(
            "{signed_info_start}{}</ds:SignedInfo>",
            between(signed, &signed_info_start, "</ds:SignedInfo>")
        );
        let canonical_signed_info =
            canonicalize(&signed_info, CanonicalizationMode::ExclusiveCanonical1_0).unwrap();

        let certificate =
            X509::from_der(&base64("<ds:X509Certificate>", "</ds:X509Certificate>")).unwrap();
        let public_key = certificate.public_key().unwrap();
        let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key).unwrap();
        verifier.update(canonical_signed_info.as_bytes()).unwrap();
        let signature_value = base64("<ds:SignatureValue>", "</ds:SignatureValue>");
        assert!(
            verifier.verify(&signature_value).unwrap(),
            "Invalid SignatureValue"
        );
    }

    #[test]
    fn read_pem_keys() {
        let certificate = fs::read_to_string(format!("{RESOURCES}/public.cer")).unwrap();
        assert_eq!(key_pair().certificate_to_pem().unwrap(), certificate);

        // PKCS#8 private keys are accepted too
        let pkcs8 = key_pair().private_key.private_key_to_pem_pkcs8().unwrap();
        let pkcs8 = String::from_utf8(pkcs8).unwrap();
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
