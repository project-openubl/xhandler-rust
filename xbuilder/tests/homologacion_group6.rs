use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group6Test";

fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: Box::leak(format!("Item{i}").into_boxed_str()),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            ..Default::default()
        })
        .collect()
}

// Case 46: Factura con Percepcion

#[serial_test::serial]
#[tokio::test]
async fn caso46_factura1_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF40-1",
        percepcion: Some(Percepcion {
            tipo: "51",
            porcentaje: Some(dec!(0.02)),
            monto: None,
            monto_base: None,
            monto_total: None,
        }),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura1Con5Items.xml")).await;
}

// Cases 47-48: NC/ND don't include percepcion — SUNAT applies it only at the invoice level.

#[serial_test::serial]
#[tokio::test]
async fn caso47_nota_credito_factura1() {
    let mut credit_note = CreditNote {
        serie_numero: "FF40-1",
        comprobante_afectado_serie_numero: "FF40-1",
        sustento_descripcion: "Homologacion",
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura1.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso48_nota_debito_factura1() {
    let mut debit_note = DebitNote {
        serie_numero: "FF40-1",
        comprobante_afectado_serie_numero: "FF40-1",
        sustento_descripcion: "Homologacion",
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura1.xml"),
    )
    .await;
}
