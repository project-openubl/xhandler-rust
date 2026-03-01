use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group10Test";

const GRATUITA_CODES: &[&str] = &[
    "11", // GravadoRetiroPorPremio
    "12", // GravadoRetiroPorDonacion
    "13", // GravadoRetiro
    "21", // ExoneradoTransferenciaGratuita
    "31", // InafectoRetiroPorBonificacion
    "32", // InafectoRetiro
    "33", // InafectoRetiroPorMuestrasMedicas
];

fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: Box::leak(format!("Item{i}").into_boxed_str()),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            igv_tipo: Some(GRATUITA_CODES[(i - 1) % GRATUITA_CODES.len()]),
            ..Default::default()
        })
        .collect()
}

// Cases 74-78: Boletas

#[serial_test::serial]
#[tokio::test]
async fn caso74_boleta1_con_7_items() {
    let mut invoice = Invoice {
        serie_numero: "BB13-1",
        detalles: items(7),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta1Con7Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso75_boleta2_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "BB13-2",
        detalles: items(2),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta2Con2Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso76_boleta3_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "BB13-3",
        detalles: items(5),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta3Con5Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso77_boleta4_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "BB13-4",
        detalles: items(4),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta4Con4Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso78_boleta5_con_9_items() {
    let mut invoice = Invoice {
        serie_numero: "BB13-5",
        detalles: items(9),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta5Con9Items.xml")).await;
}

// Cases 79-81: Notas de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso79_nota_credito_boleta1() {
    let mut credit_note = CreditNote {
        serie_numero: "BB13-1",
        comprobante_afectado_serie_numero: "BB13-1",
        sustento_descripcion: "Homologacion",
        detalles: items(7),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeBoleta1.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso80_nota_credito_boleta2() {
    let mut credit_note = CreditNote {
        serie_numero: "BB13-2",
        comprobante_afectado_serie_numero: "BB13-2",
        sustento_descripcion: "Homologacion",
        detalles: items(2),
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
async fn caso81_nota_credito_boleta4() {
    let mut credit_note = CreditNote {
        serie_numero: "BB13-3",
        comprobante_afectado_serie_numero: "BB13-4",
        sustento_descripcion: "Homologacion",
        detalles: items(4),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeBoleta4.xml"),
    )
    .await;
}

// Cases 82-84: Notas de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso82_nota_debito_boleta1() {
    let mut debit_note = DebitNote {
        serie_numero: "BB13-1",
        comprobante_afectado_serie_numero: "BB13-1",
        sustento_descripcion: "Homologacion",
        detalles: items(7),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeBoleta1.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso83_nota_debito_boleta2() {
    let mut debit_note = DebitNote {
        serie_numero: "BB13-2",
        comprobante_afectado_serie_numero: "BB13-2",
        sustento_descripcion: "Homologacion",
        detalles: items(2),
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
async fn caso84_nota_debito_boleta4() {
    let mut debit_note = DebitNote {
        serie_numero: "BB13-3",
        comprobante_afectado_serie_numero: "BB13-4",
        sustento_descripcion: "Homologacion",
        detalles: items(4),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeBoleta4.xml"),
    )
    .await;
}
