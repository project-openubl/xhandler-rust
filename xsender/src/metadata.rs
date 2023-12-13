use xml::reader::XmlEvent;
use xml::EventReader;

use crate::xml::Xml;

const CBC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
const CAC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
const SAC_NS: &str = "urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1";

#[derive(Debug)]
pub struct Metadata {
    pub document_type: String,
    pub document_id: Option<String>,
    pub ruc: Option<String>,
    pub voided_line_document_type_code: Option<String>,
}

pub trait MetadataProvider {
    fn get_metadata(&self) -> Result<Metadata, &'static str>;
}

impl MetadataProvider for Xml {
    fn get_metadata(&self) -> Result<Metadata, &'static str> {
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

        for event in event_reader {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // Document Type
                    if document_type.is_none() {
                        document_type = Some(name.local_name.clone());
                    }

                    if let (Some(namespace), Some(prefix), local_name) = (
                        name.namespace.as_deref(),
                        name.prefix.as_deref(),
                        name.local_name,
                    ) {
                        match (namespace, prefix, local_name.as_str()) {
                            (CAC_NS, "cac", "AccountingSupplierParty") => {
                                current_wrapper = Some(Wrapper::AccountingSupplierParty);
                            }
                            (SAC_NS, "sac", "VoidedDocumentsLine") => {
                                current_wrapper = Some(Wrapper::VoidedDocumentsLine);
                            }
                            _ => {}
                        };
                    };
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if let (Some(namespace), Some(prefix), local_name) = (
                        name.namespace.as_deref(),
                        name.prefix.as_deref(),
                        name.local_name,
                    ) {
                        match (&current_wrapper, namespace, prefix, local_name.as_str()) {
                            // Document ID
                            (None, CBC_NS, "cbc", "ID") => {
                                if document_id.is_none() {
                                    document_id = Some(current_text.clone());
                                }
                            }

                            // RUC
                            (Some(wrapper), CBC_NS, "cbc", "ID")
                            | (Some(wrapper), CBC_NS, "cbc", "CustomerAssignedAccountID") => {
                                if let (Wrapper::AccountingSupplierParty, None) = (wrapper, &ruc) {
                                    ruc = Some(current_text.clone());
                                };
                            }

                            // Voided line document_type_code
                            (Some(wrapper), CBC_NS, "cbc", "DocumentTypeCode") => {
                                if let (Wrapper::VoidedDocumentsLine, None) =
                                    (wrapper, &voided_line_document_type_code)
                                {
                                    voided_line_document_type_code = Some(current_text.clone());
                                };
                            }
                            _ => {}
                        };
                    };
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

        match document_type {
            Some(document_type) => Ok(Metadata {
                document_type,
                document_id,
                ruc,
                voided_line_document_type_code,
            }),
            None => Err("document_type could not be identified"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::metadata::MetadataProvider;
    use crate::xml::{FromPath, Xml};

    const RESOURCES: &str = "resources/test";

    #[test]
    fn metadata() {
        let file1 = Xml::from_path(Path::new(&format!("{RESOURCES}/F001-1.xml")));
        let metadata1 = file1.unwrap().get_metadata().unwrap();
        assert_eq!(metadata1.document_type, "Invoice");
        assert_eq!(metadata1.document_id.as_deref(), Some("F001-1"));
        assert_eq!(metadata1.ruc.as_deref(), Some("12345678912"));
        assert_eq!(metadata1.voided_line_document_type_code, None);

        let file2 = Xml::from_path(Path::new(&format!("{RESOURCES}/RA-20200328-1.xml")));
        let metadata2 = file2.unwrap().get_metadata().unwrap();
        assert_eq!(metadata2.document_type, "VoidedDocuments");
        assert_eq!(metadata2.document_id.as_deref(), Some("RA-20200328-1"));
        assert_eq!(metadata2.ruc.as_deref(), Some("12345678912"));
        assert_eq!(
            metadata2.voided_line_document_type_code.as_deref(),
            Some("01")
        );
    }
}
