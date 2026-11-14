use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_anticipos";

#[serial_test::serial]
#[tokio::test]
async fn invoice_anticipos() {
    let mut invoice = Invoice {
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
        anticipos: vec![Anticipo {
            comprobante_serie_numero: "F001-2".into(),
            monto: dec!(100),
            tipo: None,
            comprobante_tipo: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/min_anticipos.xml")).await;
}
