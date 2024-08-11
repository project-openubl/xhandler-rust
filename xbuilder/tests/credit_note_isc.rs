use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/creditnote/CreditNoteIscTest";

#[serial_test::serial]
#[tokio::test]
async fn credit_note_sistema_al_valor() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.17)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/isc_sistemaAlValor.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn credit_note_aplication_al_monto_fijo() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.20)),
                isc_tipo: Some(Catalog8::AplicacionAlMontoFijo.code()),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/isc_aplicacionAlMontoFijo.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn credit_note_sistema_de_precios_de_venta_al_publico() {
    let mut credit_note = CreditNote {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tasa: Some(dec!(0.10)),
                isc_tipo: Some(Catalog8::SistemaDePreciosDeVentaAlPublico.code()),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..credit_note_base()
    };

    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/isc_sistemaDePreciosDeVentalAlPublico.xml"),
    )
    .await;
}
