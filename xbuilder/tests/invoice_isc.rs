use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceIscTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_sistema_al_valor() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_sistemaAlValor.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_aplication_al_monto_fijo() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::AplicacionAlMontoFijo.code()),
                isc_tasa: Some(dec!(0.20)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/isc_aplicacionAlMontoFijo.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_sistem_de_precios_de_venta_al_publico() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaDePreciosDeVentaAlPublico.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/isc_sistemaDePreciosDeVentalAlPublico.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_precio_con_impuestos() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio_con_impuestos: Some(dec!(138.06)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                icb_aplica: true,
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio_con_impuestos: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                icb_aplica: true,
                igv_tipo: Some(Catalog7::GravadoRetiroPorPremio.code()),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_precioConImpuestos.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_mixed_tipo_igv() {
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
                igv_tipo: Some(Catalog7::GravadoRetiroPorPremio.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item3",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::ExoneradoOperacionOnerosa.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item4",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::ExoneradoTransferenciaGratuita.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item5",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::InafectoOperacionOnerosa.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item6",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::InafectoRetiroPorBonificacion.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_mixedTipoIgv.xml")).await;
}
