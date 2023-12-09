use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, detalle_base};
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceOrdeDeCompraTest";

#[test]
fn invoice_custom_moneda() {
    let mut invoice = Invoice {
        orden_de_compra: Some("123456"),
        detalles: vec![
            Detalle {
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/ordenDeCompra.xml"));
}
