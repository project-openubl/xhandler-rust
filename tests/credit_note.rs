use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteTest";

#[test]
fn credit_note() {
    let mut credit_note = CreditNote {
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
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/MinData_RUC.xml"));
}
