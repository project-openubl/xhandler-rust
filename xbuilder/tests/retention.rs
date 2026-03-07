use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::{assert_retention, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/retention/RetentionTest";

fn retention_simple() -> Retention {
    Retention {
        serie: "R001".into(),
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
        tipo_regimen_porcentaje: dec!(3),
        importe_total_retenido: dec!(10),
        importe_total_pagado: dec!(200),
        operacion: Some(PercepcionRetencionOperacion {
            numero_operacion: 1,
            fecha_operacion: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
            importe_operacion: dec!(100),
            comprobante: PercepcionRetencionComprobanteAfectado {
                tipo_comprobante: "01".into(),
                serie_numero: "F001-1".into(),
                fecha_emision: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
                importe_total: dec!(210),
                moneda: Some("PEN".into()),
            },
            tipo_cambio: None,
        }),
        ..Default::default()
    }
}

#[serial_test::serial]
#[tokio::test]
async fn retention_simple_test() {
    let mut doc = retention_simple();
    assert_retention(&mut doc, &format!("{BASE}/retention_simple.xml")).await;
}
