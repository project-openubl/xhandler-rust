use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, detalle_base};
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDescuentosTest";

#[test]
fn invoice_anticipos() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            precio: Some(dec!(100)),
            ..detalle_base("Item1", dec!(1))
        }],
        descuentos: vec![Descuento {
            monto: dec!(50),
            tipo: None,
            monto_base: None,
            factor: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/descuentoGlobal.xml"));
}
