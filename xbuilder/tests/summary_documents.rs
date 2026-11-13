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
                tipo_operacion: "1".into(),
                comprobante: SummaryDocumentsItemComprobante {
                    tipo_comprobante: "03".into(),
                    serie_numero: "B001-1".into(),
                    cliente: SummaryDocumentsCliente {
                        numero_documento_identidad: "12345678".into(),
                        tipo_documento_identidad: "1".into(),
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
            SummaryDocumentsItem {
                tipo_operacion: "1".into(),
                comprobante: SummaryDocumentsItemComprobante {
                    tipo_comprobante: "07".into(),
                    serie_numero: "BC02-2".into(),
                    cliente: SummaryDocumentsCliente {
                        numero_documento_identidad: "12345678".into(),
                        tipo_documento_identidad: "1".into(),
                    },
                    comprobante_afectado: Some(SummaryDocumentsComprobanteAfectado {
                        tipo_comprobante: "03".into(),
                        serie_numero: "B002-2".into(),
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        ],
        ..Default::default()
    };

    assert_summary_documents(&mut doc, &format!("{BASE}/summaryDocuments.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn summary_documents_anular_boleta_exonerada() {
    let mut doc = SummaryDocuments {
        numero: 1,
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2019, 12, 22),
        proveedor: proveedor_base(),
        comprobantes: vec![SummaryDocumentsItem {
            tipo_operacion: Catalog19::Anulado.code().into(),
            comprobante: SummaryDocumentsItemComprobante {
                tipo_comprobante: "03".into(),
                serie_numero: "B001-1".into(),
                cliente: SummaryDocumentsCliente {
                    numero_documento_identidad: "12345678".into(),
                    tipo_documento_identidad: Catalog6::DNI.code().into(),
                },
                valor_venta: SummaryDocumentsValorVenta {
                    importe_total: dec!(100),
                    gravado: Some(dec!(0)),
                    exonerado: Some(dec!(0)),
                    ..Default::default()
                },
                impuestos: SummaryDocumentsImpuestos {
                    igv: Some(dec!(0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }],
        ..Default::default()
    };

    assert_summary_documents(
        &mut doc,
        &format!("{BASE}/summaryDocuments_anularBoletaExonerada.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn summary_documents_percepcion_and_other_taxes() {
    let mut doc = SummaryDocuments {
        numero: 1,
        fecha_emision: NaiveDate::from_ymd_opt(2019, 12, 24),
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2019, 12, 22),
        proveedor: proveedor_base(),
        comprobantes: vec![SummaryDocumentsItem {
            tipo_operacion: "1".into(),
            comprobante: SummaryDocumentsItemComprobante {
                tipo_comprobante: "03".into(),
                serie_numero: "B001-1".into(),
                cliente: SummaryDocumentsCliente {
                    numero_documento_identidad: "12345678".into(),
                    tipo_documento_identidad: "1".into(),
                },
                percepcion: Some(SummaryDocumentsPercepcion {
                    tipo_regimen: Catalog22::VentaInterna.code().into(),
                    tasa: dec!(2),
                    monto_base: dec!(130),
                    monto: dec!(2.6),
                    monto_total: dec!(132.6),
                }),
                valor_venta: SummaryDocumentsValorVenta {
                    importe_total: dec!(130),
                    gravado: Some(dec!(100)),
                    ..Default::default()
                },
                impuestos: SummaryDocumentsImpuestos {
                    igv: Some(dec!(18)),
                    igv_tasa: Some(dec!(18)),
                    isc: Some(dec!(10)),
                    ivap: Some(dec!(1)),
                    otros: Some(dec!(1)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }],
        ..Default::default()
    };

    assert_summary_documents(&mut doc, &format!("{BASE}/summaryDocuments_percepcion.xml")).await;
}
