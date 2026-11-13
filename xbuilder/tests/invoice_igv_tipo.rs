use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice_offline;
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

// One invoice per Catalog7 value. Sending all of them to SUNAT beta back to
// back gets rate limited (HTTP 401), so these only check snapshot + XSD.
#[test]
fn invoice_precio_unitario() {
    for catalog7 in CATALOG7_VARIANTS {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                descripcion: "Item1".into(),
                cantidad: Decimal::ONE,
                precio: Some(dec!(100)),
                igv_tipo: Some(catalog7.code().into()),
                ..Default::default()
            }],
            ..invoice_base()
        };

        assert_invoice_offline(&mut invoice, &format!("{BASE}/invoice_pu_{catalog7}.xml"));
    }
}

#[test]
fn invoice_precio_con_impuestos() {
    for catalog7 in CATALOG7_VARIANTS {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                descripcion: "Item1".into(),
                cantidad: Decimal::ONE,
                precio_con_impuestos: Some(dec!(118)),
                igv_tipo: Some(catalog7.code().into()),
                ..Default::default()
            }],
            ..invoice_base()
        };

        assert_invoice_offline(&mut invoice, &format!("{BASE}/invoice_pr_{catalog7}.xml"));
    }
}

#[test]
fn invoice_precio_con_impuestos_and_icb() {
    for catalog7 in CATALOG7_VARIANTS {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                descripcion: "Item1".into(),
                cantidad: Decimal::ONE,
                precio_con_impuestos: Some(dec!(118)),
                igv_tipo: Some(catalog7.code().into()),
                icb_aplica: true,
                ..Default::default()
            }],
            ..invoice_base()
        };

        assert_invoice_offline(
            &mut invoice,
            &format!("{BASE}/invoice_pr_icb_{catalog7}.xml"),
        );
    }
}

#[test]
fn invoice_precio_con_impuestos_and_icb_and_isc() {
    // SUNAT rejects ISC together with IVAP
    for catalog7 in CATALOG7_VARIANTS
        .iter()
        .filter(|catalog7| !matches!(catalog7, Catalog7::GravadoIvap))
    {
        let mut invoice = Invoice {
            detalles: vec![Detalle {
                descripcion: "Item1".into(),
                cantidad: Decimal::ONE,
                precio_con_impuestos: Some(dec!(100)),
                igv_tipo: Some(catalog7.code().into()),
                icb_aplica: true,
                isc_tasa: Some(dec!(0.10)),
                ..Default::default()
            }],
            ..invoice_base()
        };

        assert_invoice_offline(
            &mut invoice,
            &format!("{BASE}/invoice_pr_icb_isc_{catalog7}.xml"),
        );
    }
}
