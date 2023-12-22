use serde::Serialize;
use tera::{Context, Result};

use crate::constants::TEMPLATES;

pub struct EnvelopeData {
    pub username: String,
    pub password: String,
    pub body: BodyData,
}

#[allow(dead_code)]
pub enum BodyData {
    SendBill(FileData),
    SendSummary(FileData),
    VerifyTicket(String),
    GetStatusCrd(DocumentData),
    ValidateCdpCriterios(DocumentCdpData),
    ValidateFile(FileData),
}

#[derive(Clone, Debug, Serialize)]
pub struct FileData {
    pub filename: String,
    pub file_content: String,
}

#[derive(Serialize)]
pub struct DocumentData {
    ruc: String,
    tipo_comprobante: String,
    serie_comprobante: String,
    numero_comprobante: String,
}

#[derive(Serialize)]
pub struct DocumentCdpData {
    ruc: String,
    tipo_comprobante: String,
    serie_comprobante: String,
    numero_comprobante: String,
    tipo_documento_identidad_receptor: String,
    numero_documento_identidad_receptor: String,
    fecha_emision: String,
    importe_total: String,
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
            BodyData::SendBill(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("send_bill.xml", &context)
            }
            BodyData::SendSummary(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("send_summary.xml", &context)
            }
            BodyData::VerifyTicket(ticket) => {
                context.insert("ticket", ticket);
                TEMPLATES.render("verify_ticket.xml", &context)
            }
            BodyData::GetStatusCrd(document_data) => {
                context.insert("body", document_data);
                TEMPLATES.render("get_status_crd.xml", &context)
            }
            BodyData::ValidateCdpCriterios(document_data) => {
                context.insert("body", document_data);
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
    use crate::soap::envelope::BodyData::{
        GetStatusCrd, SendBill, SendSummary, ValidateCdpCriterios, ValidateFile, VerifyTicket,
    };
    use crate::soap::envelope::*;

    #[test]
    fn send_bill() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendBill(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }

    #[test]
    fn send_summary() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendSummary(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
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

    #[test]
    fn get_status_crd() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: GetStatusCrd(DocumentData {
                ruc: String::from("my_ruc"),
                tipo_comprobante: String::from("my_tipo"),
                serie_comprobante: String::from("my_serie"),
                numero_comprobante: String::from("my_numero"),
            }),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
    }

    #[test]
    fn validate_cdp_criterios() {
        let message = EnvelopeData {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: ValidateCdpCriterios(DocumentCdpData {
                ruc: String::from("my_ruc"),
                tipo_comprobante: String::from("my_tipo"),
                serie_comprobante: String::from("my_serie"),
                numero_comprobante: String::from("my_numero"),
                tipo_documento_identidad_receptor: String::from("my_tipo_receptor"),
                numero_documento_identidad_receptor: String::from("my_numero_receptor"),
                fecha_emision: String::from("my_fecha_emision"),
                importe_total: String::from("my_importe_total"),
            }),
        };

        assert!(message.to_string_xml().unwrap().len() > 1);
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
