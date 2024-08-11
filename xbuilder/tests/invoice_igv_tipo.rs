use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceTipoIgvTest";

static CATALOG7_VARIANTS: &[Catalog7] = &[
    Catalog7::GravadoOperacionOnerosa,
    Catalog7::GravadoRetiroPorPremio,
    Catalog7::GravadoRetiroPorDonacion,
    Catalog7::GravadoRetiro,
    Catalog7::GravadoRetiroPorPublicidad,
    Catalog7::GravadoBonificaciones,
    Catalog7::GravadoRetiroPorEntregaATrabajadores,
    Catalog7::GravadoIvap,
    Catalog7::ExoneradoOperacionOnerosa,
    Catalog7::ExoneradoTransferenciaGratuita,
    Catalog7::InafectoOperacionOnerosa,
    Catalog7::InafectoRetiroPorBonificacion,
    Catalog7::InafectoRetiro,
    Catalog7::InafectoRetiroPorMuestrasMedicas,
    Catalog7::InafectoRetiroPorConvenioColectivo,
    Catalog7::InafectoRetiroPorPremio,
    Catalog7::InafectoRetiroPorPublicidad,
    Catalog7::Exportacion,
];

#[serial_test::serial]
#[tokio::test]
async fn invoice_precio_unitario() {
    for catalog7 in CATALOG7_VARIANTS {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                descripcion: "Item1",
                cantidad: Decimal::ONE,
                precio: Some(dec!(100)),
                igv_tipo: Some(catalog7.code()),
                ..Default::default()
            }],
            ..invoice_base()
        };

        assert_invoice(&mut invoice, &format!("{BASE}/invoice_pu_{catalog7}.xml")).await;
    }
}
