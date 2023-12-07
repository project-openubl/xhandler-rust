use rust_decimal_macros::dec;
use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::detalle_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceIscTest";

#[test]
fn invoice_sistema_al_valor() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_sistemaAlValor.xml"));
}

#[test]
fn invoice_aplication_al_monto_fijo() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::AplicacionAlMontoFijo.code()),
                isc_tasa: Some(dec!(0.20)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/isc_aplicacionAlMontoFijo.xml"),
    );
}

#[test]
fn invoice_sistem_de_precios_de_venta_al_publico() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaDePreciosDeVentaAlPublico.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/isc_sistemaDePreciosDeVentalAlPublico.xml"),
    );
}

#[test]
fn invoice_precio_con_impuestos() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio_con_impuestos: Some(dec!(138.06)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                icb_aplica: true,
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio_con_impuestos: Some(dec!(100)),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.17)),
                icb_aplica: true,
                igv_tipo: Some(Catalog7::GravadoRetiroPorPremio.code()),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_precioConImpuestos.xml"));
}

#[test]
fn invoice_mixed_tipo_igv() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::GravadoRetiroPorPremio.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item2", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::ExoneradoOperacionOnerosa.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item3", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::ExoneradoTransferenciaGratuita.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item4", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::InafectoOperacionOnerosa.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item5", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                igv_tipo: Some(Catalog7::InafectoRetiroPorBonificacion.code()),
                isc_tipo: Some(Catalog8::SistemaAlValor.code()),
                isc_tasa: Some(dec!(0.10)),
                ..detalle_base("Item6", dec!(2))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/isc_mixedTipoIgv.xml"));
}
