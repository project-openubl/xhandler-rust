use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice_orden_compra";

#[serial_test::serial]
#[tokio::test]
async fn invoice_orden_compra() {
    let mut invoice = Invoice {
        orden_compra: Some("123456".into()),
        detalles: vec![
            Detalle {
                descripcion: "Item1".into(),
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2".into(),
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/orden_de_compra.xml")).await;
}
