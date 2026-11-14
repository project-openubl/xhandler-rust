use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::{invoice_base, proveedor_base};

// Regression tests for project-openubl issue #30: totals for a fractional unit
// price (10 x 6.68), with and without IGV in the price and with ICB.
const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_issue30";

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

    assert_invoice(&mut invoice, &format!("{BASE}/with_precio_unitario.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_con_igv() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
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
        &format!("{BASE}/with_precio_unitario_con_impuestos.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_unitario_and_icb() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
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

    assert_invoice(
        &mut invoice,
        &format!("{BASE}/with_precio_unitario_icb.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_issue30_precio_con_igv_and_icb() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
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
        &format!("{BASE}/with_precio_unitario_con_impuestos_icb.xml"),
    )
    .await;
}
