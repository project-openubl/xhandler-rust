use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDireccionEntregaTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_direccion_entrega_min() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        direccion_entrega: Some(Direccion {
            direccion: Some("Jr. las flores 123"),
            codigo_local: None,
            ubigeo: None,
            departamento: None,
            provincia: None,
            distrito: None,
            urbanizacion: None,
            codigo_pais: None,
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/direccionEntregaMin.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_direccion_entrega_full() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        direccion_entrega: Some(Direccion {
            ubigeo: Some("050101"),
            departamento: Some("Ayacucho"),
            provincia: Some("Huamanga"),
            distrito: Some("Jesus Nazareno"),
            codigo_local: Some("0101"),
            urbanizacion: Some("000000"),
            direccion: Some("Jr. Las piedras 123"),
            codigo_pais: Some("PE"),
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/direccionEntregaFull.xml")).await;
}
