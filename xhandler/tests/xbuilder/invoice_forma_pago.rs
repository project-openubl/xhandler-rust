use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_forma_pago";

#[serial_test::serial]
#[tokio::test]
async fn invoice_forma_pago_contado_defecto() {
    let mut invoice = Invoice {
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

    assert_invoice(&mut invoice, &format!("{BASE}/sin_forma_pago.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_forma_pago_credito() {
    let mut invoice = Invoice {
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
        forma_pago: Some(FormaPago {
            tipo: None,
            total: None,
            cuotas: vec![
                CuotaPago {
                    importe: dec!(10),
                    fecha_pago: NaiveDate::from_ymd_opt(2022, 1, 20).unwrap(),
                },
                CuotaPago {
                    importe: dec!(20),
                    fecha_pago: NaiveDate::from_ymd_opt(2022, 2, 20).unwrap(),
                },
            ],
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/con_forma_pago.xml")).await;
}
