use std::str::FromStr;

use anyhow::anyhow;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::constants::{CAC_NS, CBC_NS};

/// Response obtained from SUNAT
pub struct CdrMetadata {
    pub response_code: String,
    pub description: String,
    pub notes: Vec<String>,
}

impl FromStr for CdrMetadata {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event_reader = EventReader::from_str(s);

        enum Wrapper {
            Response,
            Note,
        }

        let mut current_wrapper: Option<Wrapper> = None;
        let mut current_text: String = String::from("");

        // Send bill data
        let mut response_code: Option<String> = None;
        let mut description: Option<String> = None;

        // Send summary data
        let mut notes: Vec<String> = Vec::new();

        fn set_wrapper(
            is_start: bool,
            name: &OwnedName,
            mut current_wrapper: Option<Wrapper>,
        ) -> Option<Wrapper> {
            let namespace = name.namespace.as_deref();
            let prefix = name.prefix.as_deref();
            let local_name = name.local_name.as_str();

            match (namespace, prefix, local_name) {
                (Some(CAC_NS), Some("cac"), "Response") => {
                    current_wrapper = if is_start {
                        Some(Wrapper::Response)
                    } else {
                        None
                    }
                }
                (Some(CBC_NS), Some("cbc"), "Note") => {
                    current_wrapper = if is_start { Some(Wrapper::Note) } else { None }
                }
                _ => {}
            };

            current_wrapper
        }

        for event in event_reader {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_wrapper = set_wrapper(true, &name, current_wrapper);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let namespace = name.namespace.as_deref();
                    let prefix = name.prefix.as_deref();
                    let local_name = name.local_name.as_str();

                    match (&current_wrapper, &namespace, &prefix, local_name) {
                        (Some(Wrapper::Response), Some(CBC_NS), Some("cbc"), "ResponseCode") => {
                            if response_code.is_none() {
                                response_code = Some(current_text.clone());
                            }
                        }
                        (Some(Wrapper::Response), Some(CBC_NS), Some("cbc"), "Description") => {
                            if description.is_none() {
                                description = Some(current_text.clone());
                            }
                        }
                        (Some(Wrapper::Note), Some(CBC_NS), Some("cbc"), "Note") => {
                            notes.push(current_text.clone());
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

        match (response_code, description) {
            (Some(response_code), Some(description)) => Ok(Self {
                description,
                response_code,
                notes,
            }),
            _ => Err(anyhow!(
                "Could not find response_code and description inside the xml"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;

    use crate::soap::cdr::CdrMetadata;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn read_cdr_without_notes() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/R-12345678901-01-F001-00000587.xml"))
                .expect("Could not read file");
        let cdr = CdrMetadata::from_str(&file_content).expect("Could not read Cdr");

        assert_eq!(cdr.response_code, "0");
        assert_eq!(
            cdr.description,
            "La Factura numero F001-00000587, ha sido aceptada"
        );
        assert!(cdr.notes.is_empty());
    }

    #[test]
    fn read_cdr_with_notes() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/R-20220557805-01-F001-22Openubl.xml"))
                .expect("Could not read file");
        let cdr = CdrMetadata::from_str(&file_content).expect("Could not read Cdr");

        assert_eq!(cdr.response_code, "0");
        assert_eq!(
            cdr.description,
            "La Factura numero F001-22, ha sido aceptada"
        );
        assert_eq!(cdr.notes.len(), 8);
        assert!(cdr
            .notes
            .last()
            .unwrap()
            .starts_with("4252 - El dato ingresado como atributo @listName es incorrecto."));
    }
}
