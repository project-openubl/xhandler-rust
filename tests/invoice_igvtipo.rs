use xbuilder::models::general::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::detalle_base;
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

#[test]
fn invoice_precio_unitario() {
    for catalog7 in CATALOG7_VARIANTS {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                cantidad: 1f64,
                precio: Some(100f64),
                igv_tipo: Some(catalog7.code()),
                ..detalle_base("Item1", 10f64)
            }],
            ..invoice_base()
        };

        assert_invoice(&mut invoice, &format!("{BASE}/invoice_pu_{catalog7}.xml"));
    }
}
