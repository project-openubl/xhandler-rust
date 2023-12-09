use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceFormaPagoTest";

#[test]
fn invoice_forma_pago_contado_defecto() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(10))
            },
        ],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/sinFormaPago.xml"));
}

#[test]
fn invoice_forma_pago_credito() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(10))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(10))
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

    assert_invoice(&mut invoice, &format!("{BASE}/conFormaPago.xml"));
}
