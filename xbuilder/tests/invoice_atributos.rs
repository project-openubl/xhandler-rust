use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceAtributosTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_with_atributos_simple() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(2),
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
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/atributosSimple.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_with_atributos_con_periodo() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            atributos: vec![
                Atributo {
                    nombre: "Garantia",
                    codigo: "WAR",
                    valor: Some("12 meses"),
                    fecha_inicio: Some(NaiveDate::from_ymd_opt(2019, 12, 24).unwrap()),
                    fecha_fin: Some(NaiveDate::from_ymd_opt(2020, 12, 24).unwrap()),
                    duracion: Some(365),
                },
                Atributo {
                    nombre: "Modelo",
                    codigo: "MOD",
                    valor: Some("Galaxy S10"),
                    fecha_inicio: None,
                    fecha_fin: None,
                    duracion: None,
                },
            ],
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/atributosConPeriodo.xml")).await;
}
