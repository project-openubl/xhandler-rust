use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoicePercepcionTest";

#[test]
fn invoice_percepcion() {
    let mut invoice = Invoice {
        percepcion: Some(Percepcion {
            tipo: "51",
            porcentaje: Some(0.02f64),
            monto: None,
            monto_base: None,
            monto_total: None,
        }),
        detalles: vec![Detalle {
            cantidad: 4f64,
            precio: Some(200f64),
            ..detalle_base("Item1", 10f64)
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/percepcion.xml"));
}
