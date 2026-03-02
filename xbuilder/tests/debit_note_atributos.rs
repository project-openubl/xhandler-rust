use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_debit_note, debit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/debitnote/DebitNoteAtributosTest";

#[serial_test::serial]
#[tokio::test]
async fn debit_note_with_atributos() {
    let mut debit_note = DebitNote {
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            atributos: vec![
                Atributo {
                    nombre: "Marca",
                    codigo: "BRA",
                    valor: Some("Samsung"),
                    fecha_inicio: None,
                    fecha_fin: None,
                    duracion: None,
                },
                Atributo {
                    nombre: "Color",
                    codigo: "COL",
                    valor: Some("Negro"),
                    fecha_inicio: None,
                    fecha_fin: None,
                    duracion: None,
                },
            ],
            ..Default::default()
        }],
        ..debit_note_base()
    };

    assert_debit_note(&mut debit_note, &format!("{BASE}/atributosSimple.xml")).await;
}
