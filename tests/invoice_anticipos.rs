use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceAnticiposTest";

#[test]
fn invoice_anticipos() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..detalle_base()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..detalle_base()
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
