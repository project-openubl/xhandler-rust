use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group8Test";

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

// Cases 52-56: Boletas

#[serial_test::serial]
#[tokio::test]
async fn caso52_boleta1_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "BB11-1".into(),
        detalles: items(4),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta1Con4Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso53_boleta2_con_7_items() {
    let mut invoice = Invoice {
        serie_numero: "BB11-2".into(),
        detalles: items(7),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta2Con7Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso54_boleta3_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "BB11-3".into(),
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta3Con5Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso55_boleta4_con_3_items() {
    let mut invoice = Invoice {
        serie_numero: "BB11-4".into(),
        detalles: items(3),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta4Con3Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso56_boleta5_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "BB11-5".into(),
        detalles: items(2),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta5Con2Items.xml")).await;
}

// Cases 57-59: Notas de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso57_nota_credito_boleta2() {
    let mut credit_note = CreditNote {
        serie_numero: "BB11-1".into(),
        comprobante_afectado_serie_numero: "BB11-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(7),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeBoleta2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso58_nota_credito_boleta3() {
    let mut credit_note = CreditNote {
        serie_numero: "BB11-2".into(),
        comprobante_afectado_serie_numero: "BB11-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeBoleta3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso59_nota_credito_boleta4() {
    let mut credit_note = CreditNote {
        serie_numero: "BB11-3".into(),
        comprobante_afectado_serie_numero: "BB11-4".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(3),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeBoleta4.xml"),
    )
    .await;
}

// Cases 60-62: Notas de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso60_nota_debito_boleta2() {
    let mut debit_note = DebitNote {
        serie_numero: "BB11-1".into(),
        comprobante_afectado_serie_numero: "BB11-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(7),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeBoleta2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso61_nota_debito_boleta3() {
    let mut debit_note = DebitNote {
        serie_numero: "BB11-2".into(),
        comprobante_afectado_serie_numero: "BB11-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeBoleta3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso62_nota_debito_boleta4() {
    let mut debit_note = DebitNote {
        serie_numero: "BB11-3".into(),
        comprobante_afectado_serie_numero: "BB11-4".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(3),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeBoleta4.xml"),
    )
    .await;
}
