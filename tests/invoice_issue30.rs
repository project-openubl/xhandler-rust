use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, detalle_base};
use crate::common::{invoice_base, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceIssue30Test";

#[test]
fn invoice_issue30() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            ruc: "12345678912",
            razon_social: "Project OpenUBL S.A.C.",
            ..proveedor_base()
        },
        detalles: vec![Detalle {
            cantidad: dec!(10),
            precio: Some(dec!(6.68)),
            ..detalle_base("Item1", dec!(10))
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/with-precioUnitario.xml"));
}
