use chrono::{NaiveDate, NaiveTime};
use rust_decimal_macros::dec;
use xbuilder::models::common::Detalle;
use xbuilder::prelude::*;

use crate::common::detalle_base;
use crate::common::invoice_base;
use crate::common::{assert_invoice, cliente_base, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceTest";

#[test]
fn invoice_custom_unidad_medida() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                precio: Some(dec!(100)),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customUnidadMedida.xml"));
}

#[test]
fn invoice_custom_fecha_emision() {
    let mut invoice = Invoice {
        fecha_emision: NaiveDate::from_ymd_opt(2019, 1, 6),
        hora_emision: NaiveTime::from_hms_opt(0, 0, 0),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customFechaEmision.xml"));
}

#[test]
fn invoice_custom_cliente_direccion_and_contacto() {
    let mut invoice = Invoice {
        cliente: Cliente {
            contacto: Some(Contacto {
                email: "carlos@gmail.com",
                telefono: "+123456789",
            }),
            direccion: Some(Direccion {
                codigo_local: Some("0101"),
                ubigeo: Some("050101"),
                departamento: Some("Ayacucho"),
                provincia: Some("Huamanga"),
                distrito: Some("Jesus Nazareno"),
                urbanizacion: Some("000000"),
                direccion: Some("Jr. Las piedras 123"),
                codigo_pais: Some("PE"),
            }),
            ..cliente_base()
        },
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/customClienteDireccionAndContacto.xml"),
    );
}

#[test]
fn invoice_custom_proveedor_direccion_and_contacto() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            contacto: Some(Contacto {
                email: "carlos@gmail.com",
                telefono: "+123456789",
            }),
            direccion: Some(Direccion {
                codigo_local: Some("0101"),
                ubigeo: Some("050101"),
                departamento: Some("Ayacucho"),
                provincia: Some("Huamanga"),
                distrito: Some("Jesus Nazareno"),
                urbanizacion: Some("000000"),
                direccion: Some("Jr. Las piedras 123"),
                codigo_pais: Some("PE"),
            }),
            ..proveedor_base()
        },
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/customProveedorDireccionAndContacto.xml"),
    );
}

#[test]
fn invoice_custom_firmante() {
    let mut invoice = Invoice {
        firmante: Some(Firmante {
            ruc: "000000000000",
            razon_social: "Wolsnut4 S.A.C.",
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customFirmante.xml"));
}

#[test]
fn invoice_with_icb_precio_unitario() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                icb_aplica: true,
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                icb_aplica: true,
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/icb.xml"));
}

#[test]
fn invoice_with_icb_precio_con_igv() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                cantidad: dec!(10),
                precio_con_impuestos: Some(dec!(118)),
                icb_aplica: true,
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                cantidad: dec!(10),
                precio_con_impuestos: Some(dec!(118)),
                icb_aplica: true,
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/icb.xml"));
}

#[test]
fn invoice_with_custom_proveedor_direccion_not_null_and_codigo_local_null() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            direccion: Some(Direccion {
                direccion: Some("Jr. las flores 123"),
                codigo_local: None,
                ubigeo: None,
                departamento: None,
                provincia: None,
                distrito: None,
                urbanizacion: None,
                codigo_pais: None,
            }),
            ..proveedor_base()
        },
        detalles: vec![
            Detalle {
                cantidad: dec!(10),
                precio: Some(dec!(118)),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                cantidad: dec!(10),
                precio: Some(dec!(118)),
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customCodigoLocal.xml"));
}
