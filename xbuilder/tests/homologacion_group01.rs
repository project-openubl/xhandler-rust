use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/group01";

/// Generates the first `n` items following the pattern: Item N has qty=N, price=N*100.
fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: format!("Item{i}"),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            ..Default::default()
        })
        .collect()
}

// Cases 1-5: Facturas (Invoice)

#[serial_test::serial]
#[tokio::test]
async fn caso1_factura_con_3_items() {
    let mut invoice = Invoice {
        serie_numero: "FF11-1".into(),
        detalles: items(3),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_1_con_3_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso2_factura_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "FF11-2".into(),
        detalles: items(2),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_2_con_2_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso3_factura_con_1_items() {
    let mut invoice = Invoice {
        serie_numero: "FF11-3".into(),
        detalles: items(1),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_3_con_1_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso4_factura_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF11-4".into(),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_4_con_5_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso5_factura_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "FF11-5".into(),
        detalles: items(4),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura_5_con_4_items.xml")).await;
}

// Cases 6-8: Notas de Credito (CreditNote)

#[serial_test::serial]
#[tokio::test]
async fn caso6_nota_credito_factura2() {
    let mut credit_note = CreditNote {
        serie_numero: "FF11-1".into(),
        comprobante_afectado_serie_numero: "FF11-2".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(2),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_factura_2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso7_nota_credito_factura3() {
    let mut credit_note = CreditNote {
        serie_numero: "FF11-2".into(),
        comprobante_afectado_serie_numero: "FF11-3".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(1),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_factura_3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso8_nota_credito_factura4() {
    let mut credit_note = CreditNote {
        serie_numero: "FF11-3".into(),
        comprobante_afectado_serie_numero: "FF11-4".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_factura_4.xml"),
    )
    .await;
}

// Cases 9-11: Notas de Debito (DebitNote)

#[serial_test::serial]
#[tokio::test]
async fn caso9_nota_debito_factura2() {
    let mut debit_note = DebitNote {
        serie_numero: "FF11-1".into(),
        comprobante_afectado_serie_numero: "FF11-2".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(2),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_factura_2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso10_nota_debito_factura3() {
    let mut debit_note = DebitNote {
        serie_numero: "FF11-2".into(),
        comprobante_afectado_serie_numero: "FF11-3".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(1),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_factura_3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso11_nota_debito_factura4() {
    let mut debit_note = DebitNote {
        serie_numero: "FF11-3".into(),
        comprobante_afectado_serie_numero: "FF11-4".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_factura_4.xml"),
    )
    .await;
}
