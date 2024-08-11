use anyhow::anyhow;

use crate::constants::{
    BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX, GUIA_REMISION_REMITENTE_SERIE_REGEX,
    GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX,
};
use crate::models::Urls;
use crate::models::{
    Catalog1, DocumentType, RestFileTargetAction, SendFileTarget, SoapFileTargetAction,
    VerifyTicketTarget,
};

pub fn send_file_target(
    document_type: &str,
    voided_line_document_type_code: &Option<String>,
    urls: &Urls,
) -> Option<SendFileTarget> {
    match document_type {
        DocumentType::INVOICE | DocumentType::CREDIT_NOTE | DocumentType::DEBIT_NOTE => Some(
            SendFileTarget::Soap(urls.invoice.clone(), SoapFileTargetAction::Bill),
        ),
        DocumentType::SUMMARY_DOCUMENTS => Some(SendFileTarget::Soap(
            urls.invoice.clone(),
            SoapFileTargetAction::Summary,
        )),
        DocumentType::VOIDED_DOCUMENTS => match voided_line_document_type_code.as_deref() {
            Some(Catalog1::RETENCION) | Some(Catalog1::PERCEPCION) => Some(SendFileTarget::Soap(
                urls.perception_retention.clone(),
                SoapFileTargetAction::Summary,
            )),
            Some(Catalog1::GUIA_REMISION_REMITENTE) => None,
            Some(_) => Some(SendFileTarget::Soap(
                urls.invoice.clone(),
                SoapFileTargetAction::Summary,
            )),
            _ => None,
        },
        DocumentType::PERCEPTION | DocumentType::RETENTION => Some(SendFileTarget::Soap(
            urls.perception_retention.clone(),
            SoapFileTargetAction::Bill,
        )),
        DocumentType::DESPATCH_ADVICE => Some(SendFileTarget::Rest(
            urls.despatch.clone(),
            RestFileTargetAction::SendDocument,
        )),
        _ => None,
    }
}

pub fn verify_ticket_target(
    document_type: &str,
    voided_line_document_type_code: &Option<String>,
    urls: &Urls,
) -> Option<VerifyTicketTarget> {
    match document_type {
        DocumentType::VOIDED_DOCUMENTS => match voided_line_document_type_code.as_deref() {
            Some(Catalog1::RETENCION) | Some(Catalog1::PERCEPCION) => {
                Some(VerifyTicketTarget::Soap(urls.perception_retention.clone()))
            }
            Some(Catalog1::GUIA_REMISION_REMITENTE) => None,
            Some(_) => Some(VerifyTicketTarget::Soap(urls.invoice.clone())),
            _ => None,
        },
        DocumentType::SUMMARY_DOCUMENTS => Some(VerifyTicketTarget::Soap(urls.invoice.clone())),
        DocumentType::DESPATCH_ADVICE => Some(VerifyTicketTarget::Rest(urls.despatch.clone())),
        _ => None,
    }
}

pub fn filename_formatted_without_extension(
    document_type: &str,
    document_id: &str,
    ruc: &str,
) -> anyhow::Result<String> {
    match document_type {
        DocumentType::INVOICE => {
            if FACTURA_SERIE_REGEX.as_ref()?.is_match(document_id) {
                Ok(format!("{ruc}-{}-{document_id}", Catalog1::FACTURA))
            } else if BOLETA_SERIE_REGEX.as_ref()?.is_match(document_id) {
                Ok(format!("{ruc}-{}-{document_id}", Catalog1::BOLETA))
            } else {
                Err(anyhow!("Could not build filename from Invoice"))
            }
        }
        DocumentType::CREDIT_NOTE => Ok(format!("{ruc}-{}-{document_id}", Catalog1::NOTA_CREDITO)),
        DocumentType::DEBIT_NOTE => Ok(format!("{ruc}-{}-{document_id}", Catalog1::NOTA_DEBITO)),
        DocumentType::VOIDED_DOCUMENTS | DocumentType::SUMMARY_DOCUMENTS => {
            Ok(format!("{ruc}-{document_id}"))
        }
        DocumentType::PERCEPTION => Ok(format!("{ruc}-{}-{document_id}", Catalog1::PERCEPCION)),
        DocumentType::RETENTION => Ok(format!("{ruc}-{}-{document_id}", Catalog1::RETENCION)),
        DocumentType::DESPATCH_ADVICE => {
            if GUIA_REMISION_REMITENTE_SERIE_REGEX
                .as_ref()?
                .is_match(document_id)
            {
                Ok(format!(
                    "{ruc}-{}-{document_id}",
                    Catalog1::GUIA_REMISION_REMITENTE
                ))
            } else if GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX
                .as_ref()?
                .is_match(document_id)
            {
                Ok(format!(
                    "{ruc}-{}-{document_id}",
                    Catalog1::GUIA_REMISION_TRANSPORTISTA
                ))
            } else {
                Err(anyhow!("Could not build filename from DespatchAdvice"))
            }
        }
        _ => Err(anyhow!("Not supported document")),
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::{send_file_target, verify_ticket_target};
    use crate::models::{
        Catalog1, DocumentType, RestFileTargetAction, SendFileTarget, SoapFileTargetAction, Urls,
        VerifyTicketTarget,
    };
    use crate::ubl_file::UblMetadata;

    lazy_static::lazy_static! {
        pub static ref DELIVERY_URLS: Urls = Urls {
            invoice: String::from("https://invoice"),
            perception_retention: String::from("https://perception_retention"),
            despatch: String::from("https://despatch"),
        };
    }

    #[test]
    fn unknown_document_type() {
        let metadata = UblMetadata {
            document_type: String::from("Unknown"),
            document_id: String::from("F001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(document_delivery, None);
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn invoice() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::INVOICE),
            document_id: String::from("F001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(String::from("https://invoice"), SoapFileTargetAction::Bill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn credit_note() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::CREDIT_NOTE),
            document_id: String::from("FC01-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(String::from("https://invoice"), SoapFileTargetAction::Bill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn debit_note() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::DEBIT_NOTE),
            document_id: String::from("FD01-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(String::from("https://invoice"), SoapFileTargetAction::Bill)
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn voided_documents() {
        // Invoice
        let baja_invoice = UblMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::FACTURA)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_invoice.document_type,
            &baja_invoice.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &baja_invoice.document_type,
            &baja_invoice.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://invoice"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://invoice"))
        );

        // CreditNote
        let baja_credit_note = UblMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::NOTA_CREDITO)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_credit_note.document_type,
            &baja_credit_note.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &baja_credit_note.document_type,
            &baja_credit_note.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://invoice"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://invoice"))
        );

        // DebitNote
        let baja_debit_note = UblMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::NOTA_DEBITO)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_debit_note.document_type,
            &baja_debit_note.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &baja_debit_note.document_type,
            &baja_debit_note.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://invoice"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://invoice"))
        );

        // Percepcion
        let baja_percepcion = UblMetadata {
            voided_line_document_type_code: Some(String::from(Catalog1::PERCEPCION)),

            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_percepcion.document_type,
            &baja_percepcion.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &baja_percepcion.document_type,
            &baja_percepcion.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://perception_retention"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://perception_retention"))
        );

        // Retencion
        let baja_retencion = UblMetadata {
            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: Some(String::from(Catalog1::RETENCION)),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_retencion.document_type,
            &baja_retencion.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &baja_retencion.document_type,
            &baja_retencion.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://perception_retention"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://perception_retention"))
        );

        // Guia
        let baja_guia = UblMetadata {
            document_type: String::from(DocumentType::VOIDED_DOCUMENTS),
            document_id: String::from("RA-20200328-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: Some(String::from(Catalog1::GUIA_REMISION_REMITENTE)),
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &baja_guia.document_type,
            &baja_guia.voided_line_document_type_code,
            &DELIVERY_URLS,
        );
        let ticket_delivery = verify_ticket_target(
            &baja_guia.document_type,
            &baja_guia.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(document_delivery, None);
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn summary_documents() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::SUMMARY_DOCUMENTS),
            document_id: String::from("S001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://invoice"),
                SoapFileTargetAction::Summary,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Soap(String::from("https://invoice"))
        );
    }

    #[test]
    fn perception() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::PERCEPTION),
            document_id: String::from("S001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://perception_retention"),
                SoapFileTargetAction::Bill,
            )
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn retention() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::RETENTION),
            document_id: String::from("R001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        );

        assert_eq!(
            document_delivery,
            SendFileTarget::Soap(
                String::from("https://perception_retention"),
                SoapFileTargetAction::Bill,
            )
        );
        assert_eq!(ticket_delivery, None);
    }

    #[test]
    fn despatch_advise() {
        let metadata = UblMetadata {
            document_type: String::from(DocumentType::DESPATCH_ADVICE),
            document_id: String::from("D001-1"),
            ruc: String::from("123456789012"),
            voided_line_document_type_code: None,
            digest_value: None,
        };

        let document_delivery = send_file_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();
        let ticket_delivery = verify_ticket_target(
            &metadata.document_type,
            &metadata.voided_line_document_type_code,
            &DELIVERY_URLS,
        )
        .unwrap();

        assert_eq!(
            document_delivery,
            SendFileTarget::Rest(
                String::from("https://despatch"),
                RestFileTargetAction::SendDocument,
            )
        );
        assert_eq!(
            ticket_delivery,
            VerifyTicketTarget::Rest(String::from("https://despatch"))
        );
    }
}
