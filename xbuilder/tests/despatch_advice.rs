use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_despatch_advice;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/despatchadvice/DespatchAdviceTest";

fn min_despatch_advice() -> DespatchAdvice {
    DespatchAdvice {
        serie_numero: "T001-1".into(),
        fecha_emision: NaiveDate::from_ymd_opt(2019, 12, 24),
        remitente: Remitente {
            ruc: "12345678912".into(),
            razon_social: "Softgreen S.A.C.".into(),
        },
        destinatario: Destinatario {
            tipo_documento_identidad: "1".into(),
            numero_documento_identidad: "12345678".into(),
            nombre: "mi cliente".into(),
        },
        envio: Envio {
            tipo_traslado: "18".into(),
            peso_total: dec!(1),
            peso_total_unidad_medida: "KG".into(),
            tipo_modalidad_traslado: "02".into(),
            fecha_traslado: NaiveDate::from_ymd_opt(2019, 12, 24).unwrap(),
            partida: Partida {
                ubigeo: "010101".into(),
                direccion: "DireccionOrigen".into(),
            },
            destino: Destino {
                ubigeo: "020202".into(),
                direccion: "DireccionDestino".into(),
            },
            ..Default::default()
        },
        detalles: vec![DespatchAdviceItem {
            unidad_medida: "KG".into(),
            cantidad: dec!(0.5),
            codigo: "123456".into(),
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[serial_test::serial]
#[tokio::test]
async fn despatch_advice_min_data() {
    let mut doc = min_despatch_advice();
    assert_despatch_advice(&mut doc, &format!("{BASE}/minData.xml")).await;
}
