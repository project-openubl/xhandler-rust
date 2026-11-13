use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::{invoice_base, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceIssue30Test";

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Project OpenUBL S.A.C.".into(),
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(6.68)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/with-precioUnitario.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_con_igv() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Project OpenUBL S.A.C.".into(),
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio_con_impuestos: Some(dec!(7.88)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/with-precioUnitarioConImpuestos.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_unitario_and_icb() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Project OpenUBL S.A.C.".into(),
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio: Some(dec!(6.68)),
            icb_aplica: true,
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/with-precioUnitario-ICB.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_con_igv_and_icb() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Project OpenUBL S.A.C.".into(),
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(10),
            precio_con_impuestos: Some(dec!(7.88)),
            icb_aplica: true,
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/with-precioUnitario-conImpuestos-ICB.xml"),
    )
    .await;
}
