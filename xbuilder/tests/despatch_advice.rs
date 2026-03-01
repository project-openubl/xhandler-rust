use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_despatch_advice;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/despatchadvice/DespatchAdviceTest";

fn min_despatch_advice() -> DespatchAdvice {
    DespatchAdvice {
        serie_numero: "T001-1",
        fecha_emision: NaiveDate::from_ymd_opt(2019, 12, 24),
        remitente: Remitente {
            ruc: "12345678912",
            razon_social: "Softgreen S.A.C.",
        },
        destinatario: Destinatario {
            tipo_documento_identidad: "1",
            numero_documento_identidad: "12345678",
            nombre: "mi cliente",
        },
        envio: Envio {
            tipo_traslado: "18",
            peso_total: dec!(1),
            peso_total_unidad_medida: "KG",
            tipo_modalidad_traslado: "02",
            fecha_traslado: NaiveDate::from_ymd_opt(2019, 12, 24).unwrap(),
            partida: Partida {
                ubigeo: "010101",
                direccion: "DireccionOrigen",
            },
            destino: Destino {
                ubigeo: "020202",
                direccion: "DireccionDestino",
            },
            ..Default::default()
        },
        detalles: vec![DespatchAdviceItem {
            unidad_medida: "KG",
            cantidad: dec!(0.5),
            codigo: "123456",
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
