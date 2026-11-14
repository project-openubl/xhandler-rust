use rust_decimal_macros::dec;

use xhandler::xbuilder::models::common::Detalle;
use xhandler::xbuilder::prelude::*;

use crate::common::{assert_debit_note, debit_note_base};

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/debit_note";

#[serial_test::serial]
#[tokio::test]
async fn debit_note() {
    let mut debit_note = DebitNote {
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
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/min_data_ruc.xml")).await;
}
