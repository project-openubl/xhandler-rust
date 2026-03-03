use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::{assert_summary_documents, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/summarydocuments/SummaryDocumentsTest";

#[serial_test::serial]
#[tokio::test]
async fn summary_documents() {
    let mut doc = SummaryDocuments {
        numero: 1,
        fecha_emision: NaiveDate::from_ymd_opt(2019, 12, 24),
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2019, 12, 22),
        proveedor: proveedor_base(),
        comprobantes: vec![
            SummaryDocumentsItem {
                tipo_operacion: "1",
                comprobante: SummaryDocumentsItemComprobante {
                    tipo_comprobante: "03",
                    serie_numero: "B001-1",
                    cliente: SummaryDocumentsCliente {
                        numero_documento_identidad: "12345678",
                        tipo_documento_identidad: "1",
                    },
                    valor_venta: SummaryDocumentsValorVenta {
                        importe_total: dec!(120),
                        gravado: Some(dec!(120)),
                        ..Default::default()
                    },
                    impuestos: SummaryDocumentsImpuestos {
                        igv: Some(dec!(18)),
                        igv_tasa: Some(dec!(18)),
                        icb: Some(dec!(2)),
                    },
                    ..Default::default()
                },
            },
            SummaryDocumentsItem {
                tipo_operacion: "1",
                comprobante: SummaryDocumentsItemComprobante {
                    tipo_comprobante: "07",
                    serie_numero: "BC02-2",
                    cliente: SummaryDocumentsCliente {
                        numero_documento_identidad: "12345678",
                        tipo_documento_identidad: "1",
                    },
                    comprobante_afectado: Some(SummaryDocumentsComprobanteAfectado {
                        tipo_comprobante: "03",
                        serie_numero: "B002-2",
                    }),
                    valor_venta: SummaryDocumentsValorVenta {
                        importe_total: dec!(118),
                        gravado: Some(dec!(118)),
                        ..Default::default()
                    },
                    impuestos: SummaryDocumentsImpuestos {
                        igv: Some(dec!(18)),
                        igv_tasa: Some(dec!(18)),
                        icb: None,
                    },
                    ..Default::default()
                },
            },
        ],
        ..Default::default()
    };

    assert_summary_documents(&mut doc, &format!("{BASE}/summaryDocuments.xml")).await;
}
