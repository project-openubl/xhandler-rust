use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, invoice_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice_cargos";

fn invoice_with_cargo(tipo: &str, monto_base: Option<Decimal>, factor: Option<Decimal>) -> Invoice {
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
            monto_base,
            factor,
        }],
        ..invoice_base()
    }
}

/// Charge added to the amount payable only (ChargeTotalAmount)
#[serial_test::serial]
#[tokio::test]
async fn invoice_cargo_que_no_afecta_base_imponible() {
    let mut invoice = invoice_with_cargo("50", Some(dec!(100)), Some(dec!(0.10)));
    assert_invoice(&mut invoice, &format!("{BASE}/cargo_no_afecta_base.xml")).await;
}

/// Charge that increases the IGV base; BaseAmount defaults to the amount
#[serial_test::serial]
#[tokio::test]
async fn invoice_cargo_que_afecta_base_imponible() {
    let mut invoice = invoice_with_cargo("49", None, None);
    assert_invoice(&mut invoice, &format!("{BASE}/cargo_afecta_base.xml")).await;
}
