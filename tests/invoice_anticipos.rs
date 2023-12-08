mod common;

use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceAnticiposTest";

#[test]
fn invoice_anticipos() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        anticipos: vec![Anticipo {
            comprobante_serie_numero: "F001-2",
            monto: dec!(100),
            tipo: None,
            comprobante_tipo: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/minAnticipos.xml"));
}
