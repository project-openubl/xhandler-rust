use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceGuiasTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_guia_serie_t() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        guias: vec![Guia {
            tipo_documento: Catalog1::GuiaRemisionRemitente.code(),
            serie_numero: "T001-1",
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/guiaSerieT.xml")).await;
}
