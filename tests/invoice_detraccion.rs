mod common;

use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDetraccionTest";

#[test]
fn invoice_detraccion() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            precio: Some(dec!(200)),
            ..detalle_base("Item1", dec!(4))
        }],
        detraccion: Some(Detraccion {
            medio_de_pago: Catalog59::DepositoEnCuenta.code(),
            cuenta_bancaria: "0004-3342343243",
            tipo_bien_detraido: "014",
            porcentaje: dec!(0.04),
            monto: None,
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/detraccion.xml"));
}
