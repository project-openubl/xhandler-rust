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
            descripcion: "Item1",
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("P001"),
            codigo_sunat: Some("43230000"),
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
            descripcion: "Item1",
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("SKU-123"),
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
            descripcion: "Item1",
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            codigo: Some("P001"),
            codigo_sunat: Some("43230000"),
            codigo_gs1: Some(CodigoGS1 {
                codigo: "8888888888888",
                tipo: "GTIN-13",
            }),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/allProductCodes.xml")).await;
}
