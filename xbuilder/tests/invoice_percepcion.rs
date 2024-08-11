use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoicePercepcionTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_percepcion() {
    let mut invoice = Invoice {
        percepcion: Some(Percepcion {
            tipo: "51",
            porcentaje: Some(dec!(0.02)),
            monto: None,
            monto_base: None,
            monto_total: None,
        }),
        detalles: vec![Detalle {
            descripcion: "Item1",
            cantidad: dec!(4),
            precio: Some(dec!(200)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/percepcion.xml")).await;
}
