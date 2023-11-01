use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceIssue30Test";

#[test]
fn invoice_issue30() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            cantidad: 10f64,
            precio: Some(6.68f64),
            ..detalle_base("Item1", 10f64)
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/with-precioUnitario.xml"));
}
