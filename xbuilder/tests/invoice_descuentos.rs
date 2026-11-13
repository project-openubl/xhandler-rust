use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDescuentosTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_descuento_global() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        descuentos: vec![Descuento {
            monto: dec!(50),
            tipo: None,
            monto_base: None,
            factor: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/descuentoGlobal.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_descuento_global_tipo02() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        descuentos: vec![Descuento {
            monto: dec!(50),
            tipo: Some(
                Catalog53::DescuentoGlobalAfectaBaseImponibleIgvIvap
                    .code()
                    .into(),
            ),
            monto_base: None,
            factor: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/descuentoGlobal_tipo02.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_descuento_global_tipo03() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        descuentos: vec![Descuento {
            monto: dec!(50),
            tipo: Some(
                Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap
                    .code()
                    .into(),
            ),
            monto_base: None,
            factor: None,
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/descuentoGlobal_tipo03.xml")).await;
}
