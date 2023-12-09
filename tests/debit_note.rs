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
                descripcion: "Item1",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..detalle_base()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..detalle_base()
            },
        ],
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/MinData_RUC.xml"));
}
