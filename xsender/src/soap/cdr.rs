use std::str::FromStr;

use anyhow::anyhow;

use crate::constants::{CAC_NS, CBC_NS};
use crate::models::Status;
use crate::xml_walker::walk_xml;

/// Response obtained from SUNAT
pub struct CdrMetadata {
    pub response_code: String,
    pub description: String,
    pub notes: Vec<String>,
}

impl CdrMetadata {
    /// Reads the CDR of a SUNAT answer: a base64 zip with the CDR XML
    pub fn from_zip_base64(base64: &str) -> anyhow::Result<Self> {
        Self::from_str(&crate::zip_archive::read_cdr_from_base64_zip(base64)?)
    }

    /// ACEPTADO, RECHAZADO, EXCEPCION... according to `response_code`
    pub fn status(&self) -> Status {
        Status::from_code_str(&self.response_code)
    }
}

impl FromStr for CdrMetadata {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut response_code: Option<String> = None;
        let mut description: Option<String> = None;
        let mut notes: Vec<String> = Vec::new();

        walk_xml(s, |path, text| match path {
            // ApplicationResponse/cac:DocumentResponse/cac:Response/cbc:*
            [_, document_response, response, element]
                if document_response.is(CAC_NS, "DocumentResponse")
                    && response.is(CAC_NS, "Response") =>
            {
                if element.is(CBC_NS, "ResponseCode") {
                    response_code.get_or_insert(text.trim().to_string());
                } else if element.is(CBC_NS, "Description") {
                    description.get_or_insert(text.trim().to_string());
                }
            }
            // ApplicationResponse/cbc:Note
            [_, element] if element.is(CBC_NS, "Note") => {
                notes.push(text.trim().to_string());
            }
            _ => {}
        })?;

        match response_code {
            Some(response_code) => Ok(Self {
                description: description.unwrap_or_default(),
                response_code,
                notes,
            }),
            None => Err(anyhow!("Could not find the response code inside the CDR")),
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
    fn read_cdr_from_zip_base64() {
        use base64::Engine;
        let xml = fs::read(format!("{RESOURCES}/R-12345678901-01-F001-00000587.xml")).unwrap();
        let zip = crate::zip_archive::create_zip(&[("R-F001-587.xml".into(), &xml)]).unwrap();
        let base64 = base64::engine::general_purpose::STANDARD.encode(zip);

        let cdr = CdrMetadata::from_zip_base64(&base64).unwrap();
        assert_eq!(cdr.response_code, "0");
    }

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

    #[test]
    fn read_cdr_without_description() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/cdr_without_description.xml"))
            .expect("Could not read file");
        let cdr = CdrMetadata::from_str(&file_content).expect("Could not read Cdr");
        assert_eq!(cdr.response_code, "2335");
        assert_eq!(cdr.description, "");
        assert_eq!(cdr.notes, vec!["4252 - Observacion".to_string()]);
        assert_eq!(cdr.status(), crate::models::Status::Rejected);
    }

    #[test]
    fn cdr_status() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/R-20220557805-01-F001-22Openubl.xml"))
                .expect("Could not read file");
        let cdr = CdrMetadata::from_str(&file_content).expect("Could not read Cdr");
        assert_eq!(cdr.status(), crate::models::Status::Accepted);
    }
}
