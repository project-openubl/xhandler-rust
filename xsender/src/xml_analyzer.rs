use crate::xml_metadata::XmlMetadata;

pub struct DocumentType {}

impl DocumentType {
    const INVOICE: &'static str = "Invoice";
    const CREDIT_NOTE: &'static str = "CreditNote";
    const DEBIT_NOTE: &'static str = "DebitNote";
    const VOIDED_DOCUMENTS: &'static str = "VoidedDocuments";
    const SUMMARY_DOCUMENTS: &'static str = "SummaryDocuments";
    const DESPATCH_ADVICE: &'static str = "DespatchAdvise";
    const PERCEPTION: &'static str = "Perception";
    const RETENTION: &'static str = "Retention";
}

pub struct Catalog1 {}

#[allow(dead_code)]
impl Catalog1 {
    const FACTURA: &'static str = "01";
    const BOLETA: &'static str = "03";
    const NOTA_CREDITO: &'static str = "07";
    const NOTA_DEBITO: &'static str = "08";
    const GUIA_REMISION_REMITENTE: &'static str = "09";
    const RETENCION: &'static str = "20";
    const PERCEPCION: &'static str = "40";
}

pub struct DeliveryUrls {
    pub invoice: String,
    pub perception_retention: String,
    pub despatch: String,
}

#[derive(Debug, PartialEq)]
pub enum Delivery {
    SOAP(String, SoapAction),
    REST(String, RestAction),
}

#[derive(Debug, PartialEq)]
pub enum SoapAction {
    SendBill,
    SendSummary,
    SendPack,
    GetStatus,
}

#[derive(Debug, PartialEq)]
pub enum RestAction {
    SendDocument,
    VerifyTicket,
}

pub trait Analyze {
    fn document(&self, urls: &DeliveryUrls) -> Option<Delivery>;
    fn ticket(&self, urls: &DeliveryUrls) -> Option<Delivery>;
}

impl Analyze for XmlMetadata {
    fn document(&self, urls: &DeliveryUrls) -> Option<Delivery> {
        match self.document_type.as_str() {
            DocumentType::INVOICE | DocumentType::CREDIT_NOTE | DocumentType::DEBIT_NOTE => {
                Some(Delivery::SOAP(urls.invoice.clone(), SoapAction::SendBill))
            }
            DocumentType::SUMMARY_DOCUMENTS => Some(Delivery::SOAP(
                urls.invoice.clone(),
                SoapAction::SendSummary,
            )),
            DocumentType::VOIDED_DOCUMENTS => {
                match self.voided_line_document_type_code.as_deref() {
                    Some(Catalog1::RETENCION) | Some(Catalog1::PERCEPCION) => Some(Delivery::SOAP(
                        urls.perception_retention.clone(),
                        SoapAction::SendSummary,
                    )),
                    Some(Catalog1::GUIA_REMISION_REMITENTE) => None,
                    Some(_) => Some(Delivery::SOAP(
                        urls.invoice.clone(),
                        SoapAction::SendSummary,
                    )),
                    _ => None,
                }
            }
            DocumentType::PERCEPTION | DocumentType::RETENTION => Some(Delivery::SOAP(
                urls.perception_retention.clone(),
                SoapAction::SendBill,
            )),
            DocumentType::DESPATCH_ADVICE => Some(Delivery::REST(
                urls.despatch.clone(),
                RestAction::SendDocument,
            )),
            _ => None,
        }
    }

    fn ticket(&self, urls: &DeliveryUrls) -> Option<Delivery> {
        match self.document_type.as_str() {
            DocumentType::VOIDED_DOCUMENTS => {
                match self.voided_line_document_type_code.as_deref() {
                    Some(Catalog1::RETENCION) | Some(Catalog1::PERCEPCION) => Some(Delivery::SOAP(
                        urls.perception_retention.clone(),
                        SoapAction::GetStatus,
                    )),
                    Some(Catalog1::GUIA_REMISION_REMITENTE) => None,
                    Some(_) => Some(Delivery::SOAP(urls.invoice.clone(), SoapAction::GetStatus)),
                    _ => None,
                }
            }
            DocumentType::SUMMARY_DOCUMENTS => {
                Some(Delivery::SOAP(urls.invoice.clone(), SoapAction::GetStatus))
            }
            DocumentType::DESPATCH_ADVICE => Some(Delivery::REST(
                urls.despatch.clone(),
                RestAction::VerifyTicket,
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::xml_analyzer::{
        Analyze, Catalog1, Delivery, DeliveryUrls, DocumentType, RestAction, SoapAction,
    };
    use crate::xml_metadata::XmlMetadata;

    #[test]
    fn unknow_document_type() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from("Unknown"),
            document_id: Some(String::from("F001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls);
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(document_delivery, None);
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn invoice() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::INVOICE),
            document_id: Some(String::from("F001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendBill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn credit_note() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::CREDIT_NOTE),
            document_id: Some(String::from("FC01-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendBill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn debit_note() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::DEBIT_NOTE),
            document_id: Some(String::from("FD01-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendBill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn voided_documents() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        // Invoice
        let baja_invoice = XmlMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::FACTURA)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
        };

        let document_delivery = baja_invoice.document(&delivery_urls).unwrap();
        let ticket_delivery = baja_invoice.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendSummary)
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::GetStatus)
        );

        // CreditNote
        let baja_credit_note = XmlMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::NOTA_CREDITO)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
        };

        let document_delivery = baja_credit_note.document(&delivery_urls).unwrap();
        let ticket_delivery = baja_credit_note.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendSummary)
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::GetStatus)
        );

        // DebitNote
        let baja_debit_note = XmlMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::NOTA_DEBITO)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
        };

        let document_delivery = baja_debit_note.document(&delivery_urls).unwrap();
        let ticket_delivery = baja_debit_note.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendSummary)
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::GetStatus)
        );

        // Percepcion
        let baja_percepcion = XmlMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::PERCEPCION)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
        };

        let document_delivery = baja_percepcion.document(&delivery_urls).unwrap();
        let ticket_delivery = baja_percepcion.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::SendSummary
            )
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::GetStatus
            )
        );

        // Retencion
        let baja_retencion = XmlMetadata {
            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: Some(String::from(Catalog1::RETENCION)),
        };

        let document_delivery = baja_retencion.document(&delivery_urls).unwrap();
        let ticket_delivery = baja_retencion.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::SendSummary
            )
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::GetStatus
            )
        );

        // Guia
        let baja_guia = XmlMetadata {
            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: Some(String::from("RA-20200328-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: Some(String::from(Catalog1::GUIA_REMISION_REMITENTE)),
        };

        let document_delivery = baja_guia.document(&delivery_urls);
        let ticket_delivery = baja_guia.ticket(&delivery_urls);

        assert_eq!(document_delivery, None);
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn summary_documents() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::SUMMARY_DOCUMENTS),
            document_id: Some(String::from("S001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::SendSummary)
        );
        assert_eq!(
            ticket_delivery,
            Delivery::SOAP(String::from("https://invoice"), SoapAction::GetStatus)
        );
    }

    #[test]
    fn perception() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::PERCEPTION),
            document_id: Some(String::from("S001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(
            document_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::SendBill
            )
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn retention() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::RETENTION),
            document_id: Some(String::from("R001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls);

        assert_eq!(
            document_delivery,
            Delivery::SOAP(
                String::from("https://perception_retention"),
                SoapAction::SendBill
            )
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn despatch_advise() {
        let delivery_urls: DeliveryUrls = DeliveryUrls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };

        let metadata = XmlMetadata {
            document_type: String::from(DocumentType::DESPATCH_ADVICE),
            document_id: Some(String::from("D001-1")),
            ruc: Some(String::from("123456789012")),
            voided_line_document_type_code: None,
        };

        let document_delivery = metadata.document(&delivery_urls).unwrap();
        let ticket_delivery = metadata.ticket(&delivery_urls).unwrap();

        assert_eq!(
            document_delivery,
            Delivery::REST(String::from("https://despatch"), RestAction::SendDocument)
        );
        assert_eq!(
            ticket_delivery,
            Delivery::REST(String::from("https://despatch"), RestAction::VerifyTicket)
        );
    }
}
