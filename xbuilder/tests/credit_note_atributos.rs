use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteAtributosTest";

#[serial_test::serial]
#[tokio::test]
async fn credit_note_with_atributos() {
    let mut credit_note = CreditNote {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(100)),
            atributos: vec![
                Atributo {
                    nombre: "Marca".into(),
                    codigo: "BRA".into(),
                    valor: Some("Samsung".into()),
                    fecha_inicio: None,
                    fecha_fin: None,
                    duracion: None,
                },
                Atributo {
                    nombre: "Color".into(),
                    codigo: "COL".into(),
                    valor: Some("Negro".into()),
                    fecha_inicio: None,
                    fecha_fin: None,
                    duracion: None,
                },
            ],
            ..Default::default()
        }],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/atributosSimple.xml")).await;
}
