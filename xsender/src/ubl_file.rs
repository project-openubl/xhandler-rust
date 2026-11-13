use std::fs;
use std::path::Path;

use crate::constants::{CAC_NS, CBC_NS, DS_NS, SAC_NS};
use crate::encoding::{decode_xml, encode_xml};
use crate::xml_walker::walk_xml;
use anyhow::anyhow;
use sha2::{Digest, Sha256};

/// A UBL XML document ready to be analyzed and sent to SUNAT.
///
/// It keeps both the decoded text (for parsing) and the exact bytes that get
/// zipped and sent. Documents declared as ISO-8859-1 are sent with their
/// original bytes, so the signed content reaches SUNAT unchanged.
pub struct UblFile {
    file_content: String,
    file_bytes: Vec<u8>,
}

impl UblFile {
    /// Reads the file from disk; see [`UblFile::from_bytes`].
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }

    /// Builds the file from its raw bytes, decoding them with the encoding
    /// declared in the XML (BOM or `<?xml encoding?>`, UTF-8 by default).
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> anyhow::Result<Self> {
        let file_bytes = bytes.into();
        let file_content = decode_xml(&file_bytes)?;
        Ok(Self {
            file_content,
            file_bytes,
        })
    }

    /// Decoded XML text
    pub fn content(&self) -> &str {
        &self.file_content
    }

    /// Bytes that are zipped and sent to SUNAT
    pub fn bytes(&self) -> &[u8] {
        &self.file_bytes
    }
}

/// Builds the file from already decoded text. The bytes sent to SUNAT are
/// encoded with the encoding the XML declares, so `encoding="ISO-8859-1"`
/// documents are sent as ISO-8859-1.
impl From<String> for UblFile {
    fn from(file_content: String) -> Self {
        let file_bytes = encode_xml(&file_content);
        Self {
            file_content,
            file_bytes,
        }
    }
}

impl From<&str> for UblFile {
    fn from(file_content: &str) -> Self {
        Self::from(file_content.to_string())
    }
}

#[derive(Debug)]
pub struct UblMetadata {
    pub document_type: String,
    pub document_id: String,
    pub ruc: String,
    pub voided_line_document_type_code: Option<String>,
    pub digest_value: Option<String>,
}

impl UblFile {
    /// Reads the data needed to name, route and track the document.
    /// Elements are matched by namespace URI, so any prefix is accepted.
    pub fn metadata(&self) -> anyhow::Result<UblMetadata> {
        let mut document_type: Option<String> = None;
        let mut document_id: Option<String> = None;
        let mut ruc: Option<String> = None;
        let mut voided_line_document_type_code: Option<String> = None;
        let mut digest_value: Option<String> = None;

        walk_xml(self.content(), |path, text| {
            let Some(element) = path.last() else {
                return;
            };
            let text = text.trim();

            if path.len() == 1 {
                document_type = Some(element.local_name.clone());
                return;
            }

            // The document ID is the root's own cbc:ID, never one nested in
            // ext:UBLExtensions, cac:Signature or any other aggregate.
            if path.len() == 2 && element.is(CBC_NS, "ID") {
                document_id.get_or_insert_with(|| text.to_string());
                return;
            }

            let inside = |namespace: &str, local_name: &str| {
                path.iter().any(|e| e.is(namespace, local_name))
            };

            if inside(DS_NS, "Signature") {
                if element.is(DS_NS, "DigestValue") {
                    digest_value.get_or_insert_with(|| text.to_string());
                }
                return;
            }

            let in_supplier = inside(CAC_NS, "AccountingSupplierParty")
                || inside(CAC_NS, "AgentParty")
                || inside(CAC_NS, "DespatchSupplierParty");
            if in_supplier
                && (element.is(CBC_NS, "ID") || element.is(CBC_NS, "CustomerAssignedAccountID"))
            {
                ruc.get_or_insert_with(|| text.to_string());
                return;
            }

            if inside(SAC_NS, "VoidedDocumentsLine") && element.is(CBC_NS, "DocumentTypeCode") {
                voided_line_document_type_code.get_or_insert_with(|| text.to_string());
            }
        })?;

        match (document_type, document_id, ruc) {
            (Some(document_type), Some(document_id), Some(ruc)) => Ok(UblMetadata {
                document_type,
                document_id,
                ruc,
                voided_line_document_type_code,
                digest_value,
            }),
            _ => Err(anyhow!(
                "document_type, document_id, and ruc were not found"
            )),
        }
    }

    pub fn sha256(&self) -> String {
        sha256_hex(&self.file_bytes)
    }
}

/// Lowercase hex SHA-256
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::ubl_file::UblFile;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn metadata() {
        let file1 = UblFile::from_path(format!("{RESOURCES}/F001-1.xml"));
        let metadata1 = file1.unwrap().metadata().unwrap();
        assert_eq!(metadata1.document_type, "Invoice");
        assert_eq!(metadata1.document_id, "F001-1");
        assert_eq!(metadata1.ruc, "12345678912");
        assert_eq!(metadata1.voided_line_document_type_code, None);
        assert_eq!(
            metadata1.digest_value.as_deref(),
            Some("dUdjbsfAegqmI/hBripxnCim3XM=")
        );

        let file2 = UblFile::from_path(format!("{RESOURCES}/RA-20200328-1.xml"));
        let metadata2 = file2.unwrap().metadata().unwrap();
        assert_eq!(metadata2.document_type, "VoidedDocuments");
        assert_eq!(metadata2.document_id, "RA-20200328-1");
        assert_eq!(metadata2.ruc, "12345678912");
        assert_eq!(
            metadata2.voided_line_document_type_code.as_deref(),
            Some("01")
        );
        assert_eq!(
            metadata2.digest_value.as_deref(),
            Some("JGeIFq43Y1Ajg3sJsOCD8faxoIM=")
        );

        let file3 = UblFile::from_path(format!("{RESOURCES}/150101-F001-11.xml"));
        let metadata3 = file3.unwrap().metadata().unwrap();
        assert_eq!(metadata3.document_type, "Invoice");
        assert_eq!(metadata3.document_id, "F001-11");
        assert_eq!(metadata3.ruc, "20602516025");
        assert_eq!(metadata3.voided_line_document_type_code, None);
        assert_eq!(
            metadata3.digest_value.as_deref(),
            Some("kLZdGqZX/X/f+g9+BerSzCT1aSU=")
        );

        let file4 = UblFile::from_path(format!("{RESOURCES}/P001-1.xml"));
        let metadata4 = file4.unwrap().metadata().unwrap();
        assert_eq!(metadata4.document_type, "Perception");
        assert_eq!(metadata4.document_id, "P001-1");
        assert_eq!(metadata4.ruc, "12345678912");
        assert_eq!(metadata4.voided_line_document_type_code, None);

        let file5 = UblFile::from_path(format!("{RESOURCES}/R001-1.xml"));
        let metadata5 = file5.unwrap().metadata().unwrap();
        assert_eq!(metadata5.document_type, "Retention");
        assert_eq!(metadata5.document_id, "R001-1");
        assert_eq!(metadata5.ruc, "12345678912");
        assert_eq!(metadata5.voided_line_document_type_code, None);
    }

    #[test]
    fn metadata_despatch_advice() {
        let file_t = UblFile::from_path(format!("{RESOURCES}/despatch-advice-T.xml"));
        let metadata_t = file_t.unwrap().metadata().unwrap();
        assert_eq!(metadata_t.document_type, "DespatchAdvice");
        assert_eq!(metadata_t.document_id, "T001-123");
        assert_eq!(metadata_t.ruc, "20123456789");
        assert_eq!(metadata_t.voided_line_document_type_code, None);

        let file_v = UblFile::from_path(format!("{RESOURCES}/despatch-advice-V.xml"));
        let metadata_v = file_v.unwrap().metadata().unwrap();
        assert_eq!(metadata_v.document_type, "DespatchAdvice");
        assert_eq!(metadata_v.document_id, "VVV1-1");
        assert_eq!(metadata_v.ruc, "20000000001");
    }

    #[test]
    fn metadata_is_prefix_independent_and_ignores_extension_ids() {
        let file = UblFile::from_path(Path::new(&format!(
            "{RESOURCES}/invoice_custom_prefixes.xml"
        )));
        let metadata = file.unwrap().metadata().unwrap();
        assert_eq!(metadata.document_type, "Invoice");
        assert_eq!(metadata.document_id, "F001-00000042");
        assert_eq!(metadata.ruc, "20123456789");
        assert_eq!(metadata.digest_value.as_deref(), Some("abc123="));
    }

    #[test]
    fn metadata_fails_on_invalid_xml() {
        let file = UblFile::from("<Invoice><cbc:ID>F001-1</Invoice>");
        assert!(file.metadata().is_err());
    }

    #[test]
    fn iso_8859_1_file_keeps_its_bytes() {
        let header = r#"<?xml version="1.0" encoding="ISO-8859-1"?>"#;
        let mut bytes = header.as_bytes().to_vec();
        bytes.extend_from_slice(
            b"<Invoice xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\" xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\">",
        );
        bytes.extend_from_slice(b"<cbc:ID>F001-1</cbc:ID><cac:AccountingSupplierParty><cbc:CustomerAssignedAccountID>20123456789</cbc:CustomerAssignedAccountID><cbc:Name>Pe\xF1a</cbc:Name></cac:AccountingSupplierParty></Invoice>");

        let file = UblFile::from_bytes(bytes.clone()).expect("ISO-8859-1 must be accepted");
        assert!(file.content().contains("Peña"));
        assert_eq!(file.bytes(), bytes.as_slice());
        assert_eq!(file.metadata().unwrap().document_id, "F001-1");

        // Decoded text goes back to the same ISO-8859-1 bytes
        let from_text = UblFile::from(file.content().to_string());
        assert_eq!(from_text.bytes(), bytes.as_slice());
    }
}
