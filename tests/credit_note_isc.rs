use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteIscTest";

#[test]
fn credit_note_sistema_al_valor() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.17)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/isc_sistemaAlValor.xml"));
}

#[test]
fn credit_note_aplication_al_monto_fijo() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.20)),
                isc_tipo: Some(Catalog8::AplicacionAlMontoFijo.code()),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/isc_aplicacionAlMontoFijo.xml"),
    );
}

#[test]
fn credit_note_sistema_de_precios_de_venta_al_publico() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.10)),
                isc_tipo: Some(Catalog8::SistemaDePreciosDeVentaAlPublico.code()),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/isc_sistemaDePreciosDeVentalAlPublico.xml"),
    );
}
