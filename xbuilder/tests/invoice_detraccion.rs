use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDetraccionTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_detraccion() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(4),
            precio: Some(dec!(200)),
            ..Default::default()
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

    assert_invoice(&mut invoice, &format!("{BASE}/detraccion.xml")).await;
}
