use rust_decimal_macros::dec;

use xhandler::xbuilder::models::common::Detalle;
use xhandler::xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/credit_note_orden_compra";

#[serial_test::serial]
#[tokio::test]
async fn credit_note_orden_compra() {
    let mut credit_note = CreditNote {
        orden_compra: Some("123456".into()),
        detalles: vec![
            Detalle {
                descripcion: "Item1".into(),
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2".into(),
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/orden_de_compra.xml")).await;
}
