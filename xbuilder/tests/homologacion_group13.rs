use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_summary_documents;
use crate::common::proveedor_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group13Test";

fn summary_items() -> Vec<SummaryDocumentsItem> {
    (1..=5)
        .map(|i| {
            let amount = dec!(100) * rust_decimal::Decimal::from(i);
            let igv = amount * dec!(0.18);
            let total = amount + igv;
            SummaryDocumentsItem {
                tipo_operacion: "1".into(),
                comprobante: SummaryDocumentsItemComprobante {
                    tipo_comprobante: "03".into(),
                    serie_numero: format!("B001-{i}"),
                    cliente: SummaryDocumentsCliente {
                        numero_documento_identidad: "12345678".into(),
                        tipo_documento_identidad: "1".into(),
                    },
                    valor_venta: SummaryDocumentsValorVenta {
                        importe_total: total,
                        gravado: Some(amount),
                        ..Default::default()
                    },
                    impuestos: SummaryDocumentsImpuestos {
                        igv: Some(igv),
                        igv_tasa: Some(dec!(18)),
                        icb: None,
                    },
                    ..Default::default()
                },
            }
        })
        .collect()
}

// Case 99: Resumen diario de Boletas de venta con 5 items

#[serial_test::serial]
#[tokio::test]
async fn caso99_resumen_diario_de_boletas() {
    let mut doc = SummaryDocuments {
        numero: 1,
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2019, 12, 22),
        proveedor: proveedor_base(),
        comprobantes: summary_items(),
        ..Default::default()
    };
    assert_summary_documents(&mut doc, &format!("{BASE}/resumenDiarioDeBoletas.xml")).await;
}
