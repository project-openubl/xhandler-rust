use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group7Test";

/// Generates `n` items with ISC (Sistema al Valor 10%), same as Group 5.
fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: Box::leak(format!("Item{i}").into_boxed_str()),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            isc_tipo: Some(Catalog8::SistemaAlValor.code()),
            isc_tasa: Some(dec!(0.10)),
            ..Default::default()
        })
        .collect()
}

// Case 49: Factura en USD con ISC

#[serial_test::serial]
#[tokio::test]
async fn caso49_factura1_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF50-1",
        moneda: Some("USD"),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura1Con5Items.xml")).await;
}

// Case 50: Nota de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso50_nota_credito_factura1() {
    let mut credit_note = CreditNote {
        serie_numero: "FF50-1",
        comprobante_afectado_serie_numero: "FF50-1",
        sustento_descripcion: "Homologacion",
        moneda: Some("USD"),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura1.xml"),
    )
    .await;
}

// Case 51: Nota de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso51_nota_debito_factura1() {
    let mut debit_note = DebitNote {
        serie_numero: "FF50-1",
        comprobante_afectado_serie_numero: "FF50-1",
        sustento_descripcion: "Homologacion",
        moneda: Some("USD"),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura2.xml"),
    )
    .await;
}
