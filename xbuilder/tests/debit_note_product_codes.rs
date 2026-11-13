use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_debit_note, debit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/debit_note_product_codes";

#[serial_test::serial]
#[tokio::test]
async fn debit_note_with_product_codes() {
    let mut debit_note = DebitNote {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            ..Default::default()
        }],
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/product_codes.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn debit_note_with_all_product_codes() {
    let mut debit_note = DebitNote {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            codigo_gs1: Some(CodigoGs1 {
                codigo: "8888888888888".into(),
                tipo: "GTIN-13".into(),
            }),
            ..Default::default()
        }],
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/all_product_codes.xml")).await;
}
