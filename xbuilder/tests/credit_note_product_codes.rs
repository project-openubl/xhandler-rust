use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteProductCodesTest";

#[serial_test::serial]
#[tokio::test]
async fn credit_note_with_product_codes() {
    let mut credit_note = CreditNote {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            ..Default::default()
        }],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/productCodes.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn credit_note_with_all_product_codes() {
    let mut credit_note = CreditNote {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            codigo_gs1: Some(CodigoGS1 {
                codigo: "8888888888888".into(),
                tipo: "GTIN-13".into(),
            }),
            ..Default::default()
        }],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/allProductCodes.xml")).await;
}
