use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::{assert_perception, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/perception/PerceptionTest";

fn perception_simple() -> Perception {
    Perception {
        serie: "P001".into(),
        numero: 1,
        fecha_emision: NaiveDate::from_ymd_opt(2022, 1, 31),
        proveedor: proveedor_base(),
        cliente: Cliente {
            tipo_documento_identidad: Catalog6::RUC.code().into(),
            numero_documento_identidad: "12121212121".into(),
            nombre: "Carlos Feria".into(),
            ..Default::default()
        },
        tipo_regimen: "01".into(),
        tipo_regimen_porcentaje: dec!(2),
        importe_total_percibido: dec!(10),
        importe_total_cobrado: dec!(210),
        operacion: Some(PercepcionRetencionOperacion {
            numero_operacion: 1,
            fecha_operacion: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
            importe_operacion: dec!(100),
            comprobante: PercepcionRetencionComprobanteAfectado {
                tipo_comprobante: "01".into(),
                serie_numero: "F001-1".into(),
                fecha_emision: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
                importe_total: dec!(200),
                moneda: Some("PEN".into()),
            },
            tipo_cambio: None,
        }),
        ..Default::default()
    }
}

#[serial_test::serial]
#[tokio::test]
async fn perception_simple_test() {
    let mut doc = perception_simple();
    assert_perception(&mut doc, &format!("{BASE}/perception_simple.xml")).await;
}
