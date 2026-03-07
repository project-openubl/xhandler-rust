use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group3Test";

/// All gratuita codes used for Group 3 items, cycling through Gravado, Exonerado, and Inafecto
/// gratuita variants.
const GRATUITA_CODES: &[&str] = &[
    "11", // GravadoRetiroPorPremio
    "12", // GravadoRetiroPorDonacion
    "13", // GravadoRetiro
    "21", // ExoneradoTransferenciaGratuita
    "31", // InafectoRetiroPorBonificacion
    "32", // InafectoRetiro
    "33", // InafectoRetiroPorMuestrasMedicas
];

/// Generates `n` gratuita items for Group 3 tests.
/// All items are gratuita (pipeline sets precio=0 and computes precio_referencia).
fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: format!("Item{i}"),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            igv_tipo: Some(GRATUITA_CODES[(i - 1) % GRATUITA_CODES.len()].into()),
            ..Default::default()
        })
        .collect()
}

// Cases 23-27: Facturas

#[serial_test::serial]
#[tokio::test]
async fn caso23_factura1_con_7_items() {
    let mut invoice = Invoice {
        serie_numero: "FF13-1".into(),
        detalles: items(7),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura1Con7Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso24_factura2_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "FF13-2".into(),
        detalles: items(2),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura2Con2Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso25_factura3_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF13-3".into(),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura3Con5Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso26_factura4_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "FF13-4".into(),
        detalles: items(4),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura4Con4Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso27_factura5_con_3_items() {
    let mut invoice = Invoice {
        serie_numero: "FF13-5".into(),
        detalles: items(3),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura5Con3Items.xml")).await;
}

// Cases 28-29: Notas de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso28_nota_credito_factura2() {
    let mut credit_note = CreditNote {
        serie_numero: "FF13-1".into(),
        comprobante_afectado_serie_numero: "FF13-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(2),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso29_nota_credito_factura3() {
    let mut credit_note = CreditNote {
        serie_numero: "FF13-2".into(),
        comprobante_afectado_serie_numero: "FF13-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura3.xml"),
    )
    .await;
}

// Cases 30-31: Notas de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso30_nota_debito_factura2() {
    let mut debit_note = DebitNote {
        serie_numero: "FF13-1".into(),
        comprobante_afectado_serie_numero: "FF13-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(2),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso31_nota_debito_factura3() {
    let mut debit_note = DebitNote {
        serie_numero: "FF13-2".into(),
        comprobante_afectado_serie_numero: "FF13-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura3.xml"),
    )
    .await;
}
