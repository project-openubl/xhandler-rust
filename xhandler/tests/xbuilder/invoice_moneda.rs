use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_moneda";

#[serial_test::serial]
#[tokio::test]
async fn invoice_custom_moneda() {
    let mut invoice = Invoice {
        moneda: Some("USD".into()),
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/custom_moneda.xml")).await;
}
