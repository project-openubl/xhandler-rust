use serde::Serialize;
use tera::{Context, Result};

use crate::constants::TEMPLATES;
use crate::models::{ConsultDocument, ValidateCdp};

pub struct EnvelopeData {
    pub username: String,
    pub password: String,
    pub body: BodyData,
}

pub enum BodyData {
    /// sendBill, sendSummary or sendPack: same body, different operation
    SendFile {
        operation: &'static str,
        file: FileData,
    },
    VerifyTicket(String),
    /// getStatus or getStatusCdr: same body, different operation
    ConsultDocument {
        operation: &'static str,
        document: ConsultDocument,
    },
    ValidateCdp(ValidateCdp),
    ValidateFile(FileData),
}

#[derive(Clone, Debug, Serialize)]
pub struct FileData {
    pub filename: String,
    pub file_content: String,
}

pub trait ToStringXml {
    fn to_string_xml(&self) -> Result<String>;
}

impl ToStringXml for EnvelopeData {
    fn to_string_xml(&self) -> Result<String> {
        let mut context = Context::new();
        context.insert("username", &self.username);
        context.insert("password", &self.password);

        match &self.body {
            BodyData::SendFile { operation, file } => {
                context.insert("operation", operation);
                context.insert("body", file);
                TEMPLATES.render("send_file.xml", &context)
            }
            BodyData::VerifyTicket(ticket) => {
                context.insert("ticket", ticket);
                TEMPLATES.render("verify_ticket.xml", &context)
            }
            BodyData::ConsultDocument {
                operation,
                document,
            } => {
                context.insert("operation", operation);
                context.insert("body", document);
                TEMPLATES.render("consult_document.xml", &context)
            }
            BodyData::ValidateCdp(cdp) => {
                context.insert("body", cdp);
                TEMPLATES.render("validate_cdp_criterios.xml", &context)
            }
            BodyData::ValidateFile(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("validate_file.xml", &context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::soap::envelope::BodyData::{SendFile, ValidateFile, VerifyTicket};
    use crate::soap::envelope::*;

    #[test]
    fn send_bill() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendFile {
                operation: "sendBill",
                file: FileData {
                    filename: String::from("my_filename"),
                    file_content: String::from("my_file_content"),
                },
            },
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }

    #[test]
    fn send_summary() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendFile {
                operation: "sendSummary",
                file: FileData {
                    filename: String::from("my_filename"),
                    file_content: String::from("my_file_content"),
                },
            },
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }

    #[test]
    fn send_pack() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendFile {
                operation: "sendPack",
                file: FileData {
                    filename: String::from("my_filename.zip"),
                    file_content: String::from("my_file_content"),
                },
            },
        };

        let xml = message.to_string_xml().unwrap();
        assert!(xml.contains("<ns2:sendPack xmlns:ns2=\"http://service.sunat.gob.pe\">"));
        assert!(xml.contains("<fileName>my_filename.zip</fileName>"));
        assert!(!xml.contains("sendBill"));
    }

    #[test]
    fn verify_ticket() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: VerifyTicket(String::from("my_ticket")),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }

    fn document() -> ConsultDocument {
        ConsultDocument {
            ruc: String::from("12345678912"),
            tipo_comprobante: String::from("01"),
            serie: String::from("F001"),
            numero: 102,
        }
    }

    #[test]
    fn get_status_cdr() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: BodyData::ConsultDocument {
                operation: "getStatusCdr",
                document: document(),
            },
        };

        let xml = message.to_string_xml().unwrap();
        assert!(xml.contains("<ns2:getStatusCdr xmlns:ns2=\"http://service.sunat.gob.pe\">"));
        assert!(xml.contains("<serieComprobante>F001</serieComprobante>"));
        assert!(xml.contains("<numeroComprobante>102</numeroComprobante>"));
    }

    #[test]
    fn validate_cdp_criterios() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: BodyData::ValidateCdp(ValidateCdp {
                document: document(),
                tipo_documento_identidad_receptor: String::from("6"),
                numero_documento_identidad_receptor: String::from("12121212121"),
                fecha_emision: String::from("31/01/2022"),
                importe_total: String::from("118.00"),
                numero_autorizacion: None,
            }),
        };

        let xml = message.to_string_xml().unwrap();
        assert!(xml.contains("<serieCDP>F001</serieCDP>"));
        // Tera escapes "/" as "&#x2F;", which XML parsers decode back
        assert!(xml.contains("<fechaEmision>31&#x2F;01&#x2F;2022</fechaEmision>"));
        assert!(xml.contains("<nroAutorizacion></nroAutorizacion>"));
    }

    #[test]
    fn validate_file() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: ValidateFile(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }
}
