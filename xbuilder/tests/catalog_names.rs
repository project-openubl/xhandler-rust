//! Catalog values can be given by name, as in xhandler-java (e.g. "DNI",
//! "GRAVADO_OPERACION_ONEROSA"), and render exactly like their codes.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::{credit_note_base, defaults_base, invoice_base};

mod common;

fn render_invoice(
    cliente_tipo: &str,
    igv_tipo: &str,
    isc_tipo: &str,
    descuento_tipo: &str,
) -> String {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            igv_tipo: Some(igv_tipo.into()),
            isc_tasa: Some(dec!(0.17)),
            isc_tipo: Some(isc_tipo.into()),
            ..Default::default()
        }],
        descuentos: vec![Descuento {
            monto: dec!(10),
            tipo: Some(descuento_tipo.into()),
            monto_base: None,
            factor: None,
        }],
        ..invoice_base()
    };
    invoice.cliente.tipo_documento_identidad = cliente_tipo.into();
    invoice.enrich(&defaults_base()).expect("Could not enrich");
    invoice.render().expect("Could not render invoice")
}

#[test]
fn invoice_accepts_catalog_names() {
    // (cliente tipo_documento_identidad, igv_tipo, isc_tipo, descuento tipo)
    let by_code = render_invoice("6", "10", "01", "02");
    let by_name = render_invoice(
        "RUC",
        "GRAVADO_OPERACION_ONEROSA",
        "sistema_al_valor",
        "DESCUENTO_GLOBAL_AFECTA_BASE_IMPONIBLE_IGV_IVAP",
    );

    assert_eq!(by_name, by_code);
    assert!(by_name.contains("<cbc:AllowanceChargeReasonCode>02</cbc:AllowanceChargeReasonCode>"));
}

#[test]
fn credit_note_accepts_catalog_names() {
    let render = |tipo_nota: &str| {
        let mut credit_note = CreditNote {
            tipo_nota: Some(tipo_nota.into()),
            detalles: vec![Detalle {
                descripcion: "Item1".into(),
                cantidad: Decimal::ONE,
                precio: Some(dec!(100)),
                ..Default::default()
            }],
            ..credit_note_base()
        };
        credit_note
            .enrich(&defaults_base())
            .expect("Could not enrich");
        credit_note.render().expect("Could not render credit note")
    };

    assert_eq!(render("DEVOLUCION_POR_ITEM"), render("07"));
}

#[test]
fn voided_documents_fill_tipo_comprobante_from_serie() {
    let mut doc = VoidedDocuments {
        comprobantes: ["F001", "B001", "T001", "V001", "P001", "R001"]
            .into_iter()
            .map(|serie| VoidedDocumentsItem {
                serie: serie.into(),
                numero: 1,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };
    doc.enrich(&defaults_base()).expect("Could not enrich");

    let tipos: Vec<_> = doc
        .comprobantes
        .iter()
        .map(|item| item.tipo_comprobante.as_deref().unwrap_or_default())
        .collect();
    assert_eq!(tipos, ["01", "03", "09", "31", "40", "20"]);
}

#[test]
fn summary_documents_accept_tipo_operacion_name() {
    let mut doc = SummaryDocuments {
        comprobantes: vec![SummaryDocumentsItem {
            tipo_operacion: "ADICIONAR".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    doc.enrich(&defaults_base()).expect("Could not enrich");

    assert_eq!(doc.comprobantes[0].tipo_operacion, "1");
}
