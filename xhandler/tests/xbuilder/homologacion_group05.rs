use rust_decimal_macros::dec;
use xhandler::xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/homologacion/group05";

/// Generates `n` items with ISC (Sistema al Valor 10%).
fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: format!("Item{i}"),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            isc_tipo: Some(Catalog8::SistemaAlValor.code().into()),
            isc_tasa: Some(dec!(0.10)),
            ..Default::default()
        })
        .collect()
}

// Case 43: Factura

#[serial_test::serial]
#[tokio::test]
async fn caso43_factura1_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF30-1".into(),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_1_con_5_items.xml")).await;
}

// Case 44: Nota de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso44_nota_credito_factura1() {
    let mut credit_note = CreditNote {
        serie_numero: "FF30-1".into(),
        comprobante_afectado_serie_numero: "FF30-1".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_factura_1.xml"),
    )
    .await;
}

// Case 45: Nota de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso45_nota_debito_factura1() {
    let mut debit_note = DebitNote {
        serie_numero: "FF30-1".into(),
        comprobante_afectado_serie_numero: "FF30-1".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_factura_1.xml"),
    )
    .await;
}
