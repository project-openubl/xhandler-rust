use chrono::{NaiveDate, NaiveTime};
use xbuilder::models::general::Detalle;
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
                precio: Some(100f64),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item1", 10f64)
            },
            Detalle {
                precio: Some(100f64),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item2", 10f64)
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
                codigo_local: "0101",
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
                codigo_local: "0101",
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
                cantidad: 10f64,
                precio: Some(100f64),
                icb_aplica: true,
                ..detalle_base("Item1", 10f64)
            },
            Detalle {
                cantidad: 10f64,
                precio: Some(100f64),
                icb_aplica: true,
                ..detalle_base("Item2", 10f64)
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/icb.xml"));
}
