use std::fs;
use std::path::Path;

use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::constants::{CAC_NS, CBC_NS, SAC_NS};

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
}

#[derive(Debug)]
pub struct UblMetadataError {
    pub message: String,
}

impl UblFile {
    pub fn metadata(&self) -> Result<UblMetadata, UblMetadataError> {
        let event_reader = EventReader::from_str(&self.file_content);

        enum Wrapper {
            AccountingSupplierParty,
            VoidedDocumentsLine,
        }

        let mut current_wrapper: Option<Wrapper> = None;
        let mut current_text: String = String::from("");

        let mut document_type: Option<String> = None;
        let mut document_id: Option<String> = None;
        let mut ruc: Option<String> = None;
        let mut voided_line_document_type_code: Option<String> = None;

        fn set_wrapper(
            is_start: bool,
            name: &OwnedName,
            mut current_wrapper: Option<Wrapper>,
        ) -> Option<Wrapper> {
            let namespace = name.namespace.as_deref();
            let prefix = name.prefix.as_deref();
            let local_name = name.local_name.as_str();

            match (namespace, prefix, local_name) {
                (Some(CAC_NS), Some("cac"), "AccountingSupplierParty") => {
                    current_wrapper = if is_start {
                        Some(Wrapper::AccountingSupplierParty)
                    } else {
                        None
                    }
                }
                (Some(SAC_NS), Some("sac"), "VoidedDocumentsLine") => {
                    current_wrapper = if is_start {
                        Some(Wrapper::VoidedDocumentsLine)
                    } else {
                        None
                    }
                }
                _ => {}
            };

            current_wrapper
        }

        for event in event_reader {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // Document Type
                    if document_type.is_none() {
                        document_type = Some(name.local_name.clone());
                    }

                    current_wrapper = set_wrapper(true, &name, current_wrapper);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let namespace = name.namespace.as_deref();
                    let prefix = name.prefix.as_deref();
                    let local_name = name.local_name.as_str();

                    match (&current_wrapper, &namespace, &prefix, local_name) {
                        (None, Some(CBC_NS), Some("cbc"), "ID") => {
                            if document_id.is_none() {
                                document_id = Some(current_text.clone());
                            }
                        }
                        (
                            Some(Wrapper::AccountingSupplierParty),
                            Some(CBC_NS),
                            Some("cbc"),
                            "ID",
                        )
                        | (
                            Some(Wrapper::AccountingSupplierParty),
                            Some(CBC_NS),
                            Some("cbc"),
                            "CustomerAssignedAccountID",
                        ) => {
                            ruc = Some(current_text.clone());
                        }
                        (
                            Some(Wrapper::VoidedDocumentsLine),
                            Some(CBC_NS),
                            Some("cbc"),
                            "DocumentTypeCode",
                        ) => {
                            voided_line_document_type_code = Some(current_text.clone());
                        }
                        _ => {}
                    };

                    current_wrapper = set_wrapper(false, &name, current_wrapper);
                }
                Ok(XmlEvent::Characters(characters)) => {
                    current_text = characters;
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
        }

        match (document_type, document_id, ruc) {
            (Some(document_type), Some(document_id), Some(ruc)) => Ok(UblMetadata {
                document_type,
                document_id,
                ruc,
                voided_line_document_type_code,
            }),
            _ => Err(UblMetadataError {
                message: "document_type, document_id, and ruc were not found".to_string(),
            }),
        }
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

        let file2 = UblFile::from_path(Path::new(&format!("{RESOURCES}/RA-20200328-1.xml")));
        let metadata2 = file2.unwrap().metadata().unwrap();
        assert_eq!(metadata2.document_type, "VoidedDocuments");
        assert_eq!(metadata2.document_id, "RA-20200328-1");
        assert_eq!(metadata2.ruc, "12345678912");
        assert_eq!(
            metadata2.voided_line_document_type_code.as_deref(),
            Some("01")
        );
    }
}
