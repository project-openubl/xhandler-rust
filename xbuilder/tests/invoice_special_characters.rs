use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::{cliente_base, invoice_base, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceSpecialCharactersTest";

/// Values inside CDATA must be emitted verbatim (no `&amp;`, `&#x2F;`...),
/// values outside CDATA must be XML-escaped.
#[serial_test::serial]
#[tokio::test]
async fn invoice_special_characters() {
    let mut invoice = Invoice {
        proveedor: Proveedor {
            razon_social: "Softgreen & Asociados S.A.C.".into(),
            ..proveedor_base()
        },
        cliente: Cliente {
            nombre: "Peña <Ñandú> 1/2".into(),
            ..cliente_base()
        },
        observaciones: Some("Entrega 50% & saldo; ver \"]]>\" en anexo".into()),
        orden_de_compra: Some("OC-1&2".into()),
        detalles: vec![Detalle {
            descripcion: "Café 'premium' & azúcar 1/2 kg".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/specialCharacters.xml")).await;
}
