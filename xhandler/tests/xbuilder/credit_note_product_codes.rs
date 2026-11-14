use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/credit_note_product_codes";

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

    assert_credit_note(&mut credit_note, &format!("{BASE}/product_codes.xml")).await;
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
            codigo_gs1: Some(CodigoGs1 {
                codigo: "8888888888888".into(),
                tipo: "GTIN-13".into(),
            }),
            ..Default::default()
        }],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/all_product_codes.xml")).await;
}
