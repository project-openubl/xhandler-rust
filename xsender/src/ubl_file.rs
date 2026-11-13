use std::fs;
use std::path::Path;

use crate::constants::{CAC_NS, CBC_NS, DS, SAC_NS};
use crate::xml_utils::walk_xml;
use anyhow::anyhow;
use sha2::{Digest, Sha256};

pub struct UblFile {
    pub file_content: String,
}

pub trait FromPath {
    fn from_path(path: &Path) -> std::io::Result<UblFile>;
}

impl FromPath for UblFile {
    fn from_path(path: &Path) -> std::io::Result<UblFile> {
        let file_content = fs::read_to_string(path)?;
        Ok(UblFile { file_content })
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

        walk_xml(&self.file_content, |path, text| {
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

            if inside(DS, "Signature") {
                if element.is(DS, "DigestValue") {
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
        let mut hasher = Sha256::new();
        hasher.update(self.file_content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::ubl_file::{FromPath, UblFile};

    const RESOURCES: &str = "resources/test";

    #[test]
    fn metadata() {
        let file1 = UblFile::from_path(Path::new(&format!("{RESOURCES}/F001-1.xml")));
        let metadata1 = file1.unwrap().metadata().unwrap();
        assert_eq!(metadata1.document_type, "Invoice");
        assert_eq!(metadata1.document_id, "F001-1");
        assert_eq!(metadata1.ruc, "12345678912");
        assert_eq!(metadata1.voided_line_document_type_code, None);
        assert_eq!(
            metadata1.digest_value.as_deref(),
            Some("dUdjbsfAegqmI/hBripxnCim3XM=")
        );

        let file2 = UblFile::from_path(Path::new(&format!("{RESOURCES}/RA-20200328-1.xml")));
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

        let file3 = UblFile::from_path(Path::new(&format!("{RESOURCES}/150101-F001-11.xml")));
        let metadata3 = file3.unwrap().metadata().unwrap();
        assert_eq!(metadata3.document_type, "Invoice");
        assert_eq!(metadata3.document_id, "F001-11");
        assert_eq!(metadata3.ruc, "20602516025");
        assert_eq!(metadata3.voided_line_document_type_code, None);
        assert_eq!(
            metadata3.digest_value.as_deref(),
            Some("kLZdGqZX/X/f+g9+BerSzCT1aSU=")
        );

        let file4 = UblFile::from_path(Path::new(&format!("{RESOURCES}/P001-1.xml")));
        let metadata4 = file4.unwrap().metadata().unwrap();
        assert_eq!(metadata4.document_type, "Perception");
        assert_eq!(metadata4.document_id, "P001-1");
        assert_eq!(metadata4.ruc, "12345678912");
        assert_eq!(metadata4.voided_line_document_type_code, None);

        let file5 = UblFile::from_path(Path::new(&format!("{RESOURCES}/R001-1.xml")));
        let metadata5 = file5.unwrap().metadata().unwrap();
        assert_eq!(metadata5.document_type, "Retention");
        assert_eq!(metadata5.document_id, "R001-1");
        assert_eq!(metadata5.ruc, "12345678912");
        assert_eq!(metadata5.voided_line_document_type_code, None);
    }

    #[test]
    fn metadata_despatch_advice() {
        let file_t = UblFile::from_path(Path::new(&format!("{RESOURCES}/despatch-advice-T.xml")));
        let metadata_t = file_t.unwrap().metadata().unwrap();
        assert_eq!(metadata_t.document_type, "DespatchAdvice");
        assert_eq!(metadata_t.document_id, "T001-123");
        assert_eq!(metadata_t.ruc, "20123456789");
        assert_eq!(metadata_t.voided_line_document_type_code, None);

        let file_v = UblFile::from_path(Path::new(&format!("{RESOURCES}/despatch-advice-V.xml")));
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
        let file = UblFile {
            file_content: "<Invoice><cbc:ID>F001-1</Invoice>".to_string(),
        };
        assert!(file.metadata().is_err());
    }
}
