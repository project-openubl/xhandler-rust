//! Ported from xhandler-java's `e2e/enricher` tests. Where Java overwrites
//! user values Rust keeps them (see the doc comments below).

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::defaults_base;

mod common;

fn enrich(mut invoice: Invoice) -> Invoice {
    invoice.enrich(&defaults_base());
    invoice
}

fn enrich_detalle(detalle: Detalle) -> Detalle {
    let mut invoice = enrich(Invoice {
        detalles: vec![detalle],
        ..Default::default()
    });
    invoice.detalles.remove(0)
}

fn cuota() -> CuotaDePago {
    CuotaDePago {
        importe: Decimal::TEN,
        fecha_pago: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
    }
}

// enrich/GeneralDocumentTest

#[test]
fn enrich_moneda() {
    let invoice = enrich(Invoice::default());
    assert_eq!(invoice.moneda.as_deref(), Some("PEN"));
}

#[test]
fn dont_enrich_moneda() {
    let invoice = enrich(Invoice {
        moneda: Some("USD".into()),
        ..Default::default()
    });
    assert_eq!(invoice.moneda.as_deref(), Some("USD"));
}

#[test]
fn enrich_fecha_emision() {
    let invoice = enrich(Invoice::default());
    assert_eq!(invoice.fecha_emision, Some(defaults_base().date));
}

#[test]
fn dont_enrich_fecha_emision() {
    let fecha_emision = NaiveDate::from_ymd_opt(1991, 1, 30);
    let invoice = enrich(Invoice {
        fecha_emision,
        ..Default::default()
    });
    assert_eq!(invoice.fecha_emision, fecha_emision);
}

#[test]
fn enrich_forma_de_pago_contado() {
    let invoice = enrich(Invoice::default());
    let forma_de_pago = invoice.forma_de_pago.unwrap();
    assert!(matches!(forma_de_pago.tipo, Some(TipoFormaDePago::Contado)));
}

#[test]
fn enrich_forma_de_pago_credito() {
    let invoice = enrich(Invoice {
        forma_de_pago: Some(FormaDePago {
            tipo: None,
            cuotas: vec![cuota()],
            total: None,
        }),
        ..Default::default()
    });
    let forma_de_pago = invoice.forma_de_pago.unwrap();
    assert!(matches!(forma_de_pago.tipo, Some(TipoFormaDePago::Credito)));
}

/// Java corrects a CREDITO without cuotas to CONTADO (and the reverse);
/// Rust keeps the user's value.
#[test]
fn dont_enrich_forma_de_pago_tipo() {
    let invoice = enrich(Invoice {
        forma_de_pago: Some(FormaDePago {
            tipo: Some(TipoFormaDePago::Contado),
            cuotas: vec![cuota()],
            total: None,
        }),
        ..Default::default()
    });
    let forma_de_pago = invoice.forma_de_pago.unwrap();
    assert!(matches!(forma_de_pago.tipo, Some(TipoFormaDePago::Contado)));
}

#[test]
fn enrich_firmante() {
    let invoice = enrich(Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Mi razón social".into(),
            ..Default::default()
        },
        ..Default::default()
    });
    let firmante = invoice.firmante.unwrap();
    assert_eq!(firmante.ruc, "12345678912");
    assert_eq!(firmante.razon_social, "Mi razón social");
}

#[test]
fn enrich_firmante_when_partially_filled() {
    let invoice = enrich(Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Mi razón social".into(),
            ..Default::default()
        },
        firmante: Some(Firmante {
            ruc: "99999999999".into(),
            razon_social: "".into(),
        }),
        ..Default::default()
    });
    let firmante = invoice.firmante.unwrap();
    assert_eq!(firmante.ruc, "99999999999");
    assert_eq!(firmante.razon_social, "Mi razón social");
}

/// The fill loop runs until no rule changes anything, so a firmante that
/// can't be completed (empty proveedor) must not count as a change.
#[test]
fn enrich_partial_firmante_with_empty_proveedor() {
    let invoice = enrich(Invoice {
        firmante: Some(Firmante {
            ruc: "1".into(),
            razon_social: "".into(),
        }),
        ..Default::default()
    });
    assert_eq!(invoice.firmante.unwrap().ruc, "1");
}

#[test]
fn enrich_proveedor_direccion_codigo_local() {
    let invoice = enrich(Invoice::default());
    let direccion = invoice.proveedor.direccion.unwrap();
    assert_eq!(direccion.codigo_local.as_deref(), Some("0000"));
}

// enrich/InvoiceTest

#[test]
fn enrich_tipo_comprobante() {
    let invoice = enrich(Invoice {
        serie_numero: "F001-1".into(),
        ..Default::default()
    });
    assert_eq!(invoice.tipo_comprobante.as_deref(), Some("01"));
}

/// Java overwrites the user's tipo_comprobante with the one of the serie;
/// Rust keeps it.
#[test]
fn dont_enrich_tipo_comprobante() {
    let invoice = enrich(Invoice {
        serie_numero: "F001-1".into(),
        tipo_comprobante: Some("03".into()),
        ..Default::default()
    });
    assert_eq!(invoice.tipo_comprobante.as_deref(), Some("03"));
}

// enrich/DetalleTest

#[test]
fn enrich_unidad_medida() {
    let detalle = enrich_detalle(Detalle::default());
    assert_eq!(detalle.unidad_medida.as_deref(), Some("NIU"));
}

#[test]
fn dont_enrich_unidad_medida() {
    let detalle = enrich_detalle(Detalle {
        unidad_medida: Some("KG".into()),
        ..Default::default()
    });
    assert_eq!(detalle.unidad_medida.as_deref(), Some("KG"));
}

#[test]
fn enrich_igv_tipo() {
    let detalle = enrich_detalle(Detalle::default());
    assert_eq!(detalle.igv_tipo.as_deref(), Some("10"));
}

#[test]
fn dont_enrich_igv_tipo() {
    let detalle = enrich_detalle(Detalle {
        igv_tipo: Some("32".into()),
        ..Default::default()
    });
    assert_eq!(detalle.igv_tipo.as_deref(), Some("32"));
}

#[test]
fn enrich_precio_referencia_tipo_precio_con_igv() {
    let detalle = enrich_detalle(Detalle {
        igv_tipo: Some("10".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio_referencia_tipo.as_deref(), Some("01"));
}

#[test]
fn enrich_precio_referencia_tipo_valor_referencial() {
    let detalle = enrich_detalle(Detalle {
        igv_tipo: Some("12".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio_referencia_tipo.as_deref(), Some("02"));
}

// process/DetalleTest

#[test]
fn enrich_precio_referencia_precio_sin_impuestos_operacion_onerosa() {
    let detalle = enrich_detalle(Detalle {
        precio: Some(Decimal::TEN),
        igv_tipo: Some("10".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio_referencia, Some(dec!(11.80)));
}

#[test]
fn enrich_precio_referencia_precio_sin_impuestos_operacion_no_onerosa() {
    let detalle = enrich_detalle(Detalle {
        precio: Some(Decimal::TEN),
        igv_tipo: Some("13".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio_referencia, Some(Decimal::TEN));
}

#[test]
fn enrich_precio_referencia_precio_con_impuestos_operacion_onerosa() {
    let detalle = enrich_detalle(Detalle {
        precio_con_impuestos: Some(dec!(11.80)),
        igv_tipo: Some("10".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio, Some(Decimal::TEN));
    assert_eq!(detalle.precio_referencia, Some(dec!(11.80)));
}

#[test]
fn enrich_precio_referencia_precio_con_impuestos_operacion_no_onerosa() {
    let detalle = enrich_detalle(Detalle {
        precio_con_impuestos: Some(dec!(11.80)),
        igv_tipo: Some("13".into()),
        ..Default::default()
    });
    assert_eq!(detalle.precio_referencia, Some(dec!(11.80)));
}

#[test]
fn enrich_icb() {
    let detalle = enrich_detalle(Detalle::default());
    assert_eq!(detalle.icb, Some(Decimal::ZERO));
}

#[test]
fn dont_enrich_icb() {
    let detalle = enrich_detalle(Detalle {
        icb: Some(Decimal::TEN),
        ..Default::default()
    });
    assert_eq!(detalle.icb, Some(Decimal::TEN));
}

#[test]
fn enrich_icb_aplica() {
    let detalle = enrich_detalle(Detalle {
        icb: Some(Decimal::TEN),
        icb_aplica: false,
        ..Default::default()
    });
    assert!(detalle.icb_aplica);
}

#[test]
fn enrich_igv_base_imponible_precio_sin_impuestos() {
    let detalle = enrich_detalle(Detalle {
        cantidad: dec!(2),
        precio: Some(Decimal::TEN),
        ..Default::default()
    });
    assert_eq!(detalle.igv_base_imponible, Some(dec!(20)));
}

#[test]
fn enrich_igv_base_imponible_precio_con_impuestos() {
    let detalle = enrich_detalle(Detalle {
        cantidad: dec!(2),
        precio_con_impuestos: Some(dec!(11.8)),
        ..Default::default()
    });
    assert_eq!(detalle.igv_base_imponible, Some(dec!(20)));
}

#[test]
fn dont_enrich_igv_base_imponible() {
    let detalle = enrich_detalle(Detalle {
        cantidad: dec!(2),
        precio: Some(Decimal::TEN),
        igv_base_imponible: Some(dec!(999)),
        ..Default::default()
    });
    assert_eq!(detalle.igv_base_imponible, Some(dec!(999)));
}

#[test]
fn enrich_igv() {
    let detalle = enrich_detalle(Detalle {
        igv_base_imponible: Some(Decimal::TEN),
        ..Default::default()
    });
    assert_eq!(detalle.igv, Some(dec!(1.8)));
}

#[test]
fn dont_enrich_igv() {
    let detalle = enrich_detalle(Detalle {
        igv_base_imponible: Some(Decimal::TEN),
        igv: Some(dec!(999)),
        ..Default::default()
    });
    assert_eq!(detalle.igv, Some(dec!(999)));
}

#[test]
fn enrich_total_impuestos() {
    let detalle = enrich_detalle(Detalle {
        igv: Some(Decimal::TEN),
        icb: Some(dec!(2)),
        isc: Some(dec!(5)),
        ..Default::default()
    });
    assert_eq!(detalle.total_impuestos, Some(dec!(17)));
}

#[test]
fn dont_enrich_total_impuestos() {
    let detalle = enrich_detalle(Detalle {
        igv: Some(Decimal::TEN),
        icb: Some(dec!(2)),
        total_impuestos: Some(dec!(999)),
        ..Default::default()
    });
    assert_eq!(detalle.total_impuestos, Some(dec!(999)));
}
