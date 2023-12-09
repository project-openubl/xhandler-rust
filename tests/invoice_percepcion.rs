use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, detalle_base};
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoicePercepcionTest";

#[test]
fn invoice_percepcion() {
    let mut invoice = Invoice {
        percepcion: Some(Percepcion {
            tipo: "51",
            porcentaje: Some(dec!(0.02)),
            monto: None,
            monto_base: None,
            monto_total: None,
        }),
        detalles: vec![Detalle {
            cantidad: dec!(4),
            precio: Some(dec!(200)),
            ..detalle_base("Item1", dec!(10))
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/percepcion.xml"));
}
