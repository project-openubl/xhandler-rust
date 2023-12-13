use serde::Serialize;
use tera::{Context, Tera};

pub struct SoapMessage {
    pub username: String,
    pub password: String,
    pub body: SoapBody,
}

pub enum SoapBody {
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

pub trait RendererSoap {
    fn render(&self) -> tera::Result<String>;
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/*.xml") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

impl RendererSoap for SoapMessage {
    fn render(&self) -> tera::Result<String> {
        let mut context = Context::new();
        context.insert("username", &self.username);
        context.insert("password", &self.password);

        match &self.body {
            SoapBody::SendBill(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("send_bill.xml", &context)
            }
            SoapBody::SendSummary(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("send_summary.xml", &context)
            }
            SoapBody::VerifyTicket(ticket) => {
                context.insert("ticket", ticket);
                TEMPLATES.render("verify_ticket.xml", &context)
            }
            SoapBody::GetStatusCrd(document_data) => {
                context.insert("body", document_data);
                TEMPLATES.render("get_status_crd.xml", &context)
            }
            SoapBody::ValidateCdpCriterios(document_data) => {
                context.insert("body", document_data);
                TEMPLATES.render("validate_cdp_criterios.xml", &context)
            }
            SoapBody::ValidateFile(file_data) => {
                context.insert("body", &file_data);
                TEMPLATES.render("validate_file.xml", &context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message_renderer::SoapBody::{
        GetStatusCrd, SendBill, SendSummary, ValidateCdpCriterios, ValidateFile, VerifyTicket,
    };
    use crate::message_renderer::{
        DocumentCdpData, DocumentData, FileData, RendererSoap, SoapMessage,
    };

    #[test]
    fn send_bill() {
        let message = SoapMessage {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendBill(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        let result = message.render();
        assert!(result.is_ok());
    }

    #[test]
    fn send_summary() {
        let message = SoapMessage {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: SendSummary(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        let result = message.render();
        assert!(result.is_ok());
    }

    #[test]
    fn verify_ticket() {
        let message = SoapMessage {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: VerifyTicket(String::from("my_ticket")),
        };

        let result = message.render();
        assert!(result.is_ok());
    }

    #[test]
    fn get_status_crd() {
        let message = SoapMessage {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: GetStatusCrd(DocumentData {
                ruc: String::from("my_ruc"),
                tipo_comprobante: String::from("my_tipo"),
                serie_comprobante: String::from("my_serie"),
                numero_comprobante: String::from("my_numero"),
            }),
        };

        let result = message.render();
        assert!(result.is_ok());
    }

    #[test]
    fn validate_cdp_criterios() {
        let message = SoapMessage {
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

        let result = message.render();
        assert!(result.is_ok());
    }

    #[test]
    fn validate_file() {
        let message = SoapMessage {
            username: String::from("my_username"),
            password: String::from("my_password"),
            body: ValidateFile(FileData {
                filename: String::from("my_filename"),
                file_content: String::from("my_file_content"),
            }),
        };

        let result = message.render();
        assert!(result.is_ok());
    }
}
