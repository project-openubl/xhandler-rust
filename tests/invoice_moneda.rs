use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceMonedaTest";

#[test]
fn invoice_custom_moneda() {
    let mut invoice = Invoice {
        moneda: Some("USD"),
        detalles: vec![Detalle {
            cantidad: 1f64,
            precio: Some(100f64),
            ..detalle_base("Item1", 10f64)
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customMoneda.xml"));
}
