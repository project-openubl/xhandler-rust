use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceMonedaTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_custom_moneda() {
    let mut invoice = Invoice {
        moneda: Some("USD"),
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..detalle_base()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/customMoneda.xml")).await;
}
