use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceProductCodesTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_with_product_codes() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/productCodes.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_with_only_seller_code() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("SKU-123".into()),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/onlySellerCode.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_with_all_product_codes() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("P001".into()),
            codigo_sunat: Some("43230000".into()),
            codigo_gs1: Some(CodigoGS1 {
                codigo: "8888888888888".into(),
                tipo: "GTIN-13".into(),
            }),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/allProductCodes.xml")).await;
}
