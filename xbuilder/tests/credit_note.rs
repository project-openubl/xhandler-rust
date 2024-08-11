use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteTest";

#[serial_test::serial]
#[tokio::test]
async fn credit_note() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/MinData_RUC.xml")).await;
}
