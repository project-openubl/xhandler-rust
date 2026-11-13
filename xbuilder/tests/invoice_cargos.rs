use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, invoice_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceCargosTest";

fn invoice_with_cargo(tipo: &str) -> Invoice {
    Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        cargos: vec![Cargo {
            tipo: tipo.into(),
            monto: dec!(10),
            monto_base: Some(dec!(100)),
            factor: Some(dec!(0.10)),
        }],
        ..invoice_base()
    }
}

/// Charge added to the amount payable only (ChargeTotalAmount)
#[serial_test::serial]
#[tokio::test]
async fn invoice_cargo_que_no_afecta_base_imponible() {
    let mut invoice = invoice_with_cargo("50");
    assert_invoice(&mut invoice, &format!("{BASE}/cargoNoAfectaBase.xml")).await;
}

/// Charge that increases the IGV base
#[serial_test::serial]
#[tokio::test]
async fn invoice_cargo_que_afecta_base_imponible() {
    let mut invoice = invoice_with_cargo("49");
    assert_invoice(&mut invoice, &format!("{BASE}/cargoAfectaBase.xml")).await;
}
