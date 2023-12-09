use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::{assert_debit_note, debit_note_base, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/debitnote/DebitNoteTest";

#[test]
fn debit_note() {
    let mut debit_note = DebitNote {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/MinData_RUC.xml"));
}
