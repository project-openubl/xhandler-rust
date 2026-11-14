use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_fecha_vencimiento";

#[serial_test::serial]
#[tokio::test]
async fn invoice_fecha_vencimiento() {
    let mut invoice = Invoice {
        fecha_vencimiento: NaiveDate::from_ymd_opt(2022, 1, 1),
        detalles: vec![
            Detalle {
                descripcion: "Item1".into(),
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2".into(),
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/con_fecha_vencimiento.xml")).await;
}
