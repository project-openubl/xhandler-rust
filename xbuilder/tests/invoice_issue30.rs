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
            ruc: "12345678912",
            razon_social: "Project OpenUBL S.A.C.",
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(10),
            precio: Some(dec!(6.68)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/with-precioUnitario.xml")).await;
}
