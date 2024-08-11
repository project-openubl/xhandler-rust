use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceFormaPagoTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_forma_pago_contado_defecto() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/sinFormaPago.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_forma_pago_credito() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(10),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        forma_de_pago: Some(FormaDePago {
            tipo: None,
            total: None,
            cuotas: vec![
                CuotaDePago {
                    importe: dec!(10),
                    fecha_pago: NaiveDate::from_ymd_opt(2022, 1, 20).unwrap(),
                },
                CuotaDePago {
                    importe: dec!(20),
                    fecha_pago: NaiveDate::from_ymd_opt(2022, 2, 20).unwrap(),
                },
            ],
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/conFormaPago.xml")).await;
}
