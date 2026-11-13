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

    assert_credit_note(&mut credit_note, &format!("{BASE}/MinData_RUC.xml")).await;
}

#[test]
fn credit_note_with_hora_emision() {
    let mut credit_note = CreditNote {
        hora_emision: chrono::NaiveTime::from_hms_opt(10, 30, 0),
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(1),
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        ..credit_note_base()
    };
    credit_note
        .enrich(&common::defaults_base())
        .expect("Could not enrich");
    let xml = credit_note.render().expect("Could not render credit note");

    assert!(xml.contains("<cbc:IssueTime>10:30:00</cbc:IssueTime>"));
}
