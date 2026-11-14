use rust_decimal_macros::dec;
use xhandler::xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/homologacion/group11";

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

fn descuento_global() -> Vec<Descuento> {
    vec![Descuento {
        tipo: Some(
            Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap
                .code()
                .into(),
        ),
        monto: dec!(100),
        monto_base: None,
        factor: None,
    }]
}

// Cases 85-89: Boletas con Descuento Global

#[serial_test::serial]
#[tokio::test]
async fn caso85_boleta1_con_10_items() {
    let mut invoice = Invoice {
        serie_numero: "BB14-1".into(),
        detalles: items(10),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta_1_con_10_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso86_boleta2_con_7_items() {
    let mut invoice = Invoice {
        serie_numero: "BB14-2".into(),
        detalles: items(7),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta_2_con_7_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso87_boleta3_con_6_items() {
    let mut invoice = Invoice {
        serie_numero: "BB14-3".into(),
        detalles: items(6),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta_3_con_6_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso88_boleta4_con_9_items() {
    let mut invoice = Invoice {
        serie_numero: "BB14-4".into(),
        detalles: items(9),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta_4_con_9_items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso89_boleta5_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "BB14-5".into(),
        detalles: items(4),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/boleta_5_con_4_items.xml")).await;
}

// Cases 90-92: Notas de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso90_nota_credito_boleta1() {
    let mut credit_note = CreditNote {
        serie_numero: "BB14-1".into(),
        comprobante_afectado_serie_numero: "BB14-1".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(10),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_boleta_1.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso91_nota_credito_boleta2() {
    let mut credit_note = CreditNote {
        serie_numero: "BB14-2".into(),
        comprobante_afectado_serie_numero: "BB14-2".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(7),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_boleta_2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso92_nota_credito_boleta4() {
    let mut credit_note = CreditNote {
        serie_numero: "BB14-3".into(),
        comprobante_afectado_serie_numero: "BB14-4".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(9),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/nota_de_credito_de_boleta_4.xml"),
    )
    .await;
}

// Cases 93-95: Notas de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso93_nota_debito_boleta1() {
    let mut debit_note = DebitNote {
        serie_numero: "BB14-1".into(),
        comprobante_afectado_serie_numero: "BB14-1".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(10),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_boleta_1.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso94_nota_debito_boleta2() {
    let mut debit_note = DebitNote {
        serie_numero: "BB14-2".into(),
        comprobante_afectado_serie_numero: "BB14-2".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(7),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_boleta_2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso95_nota_debito_boleta4() {
    let mut debit_note = DebitNote {
        serie_numero: "BB14-3".into(),
        comprobante_afectado_serie_numero: "BB14-4".into(),
        descripcion_sustento: "Homologacion".into(),
        detalles: items(9),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/nota_de_debito_de_boleta_4.xml"),
    )
    .await;
}
