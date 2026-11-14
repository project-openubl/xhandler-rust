use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_guias";

#[serial_test::serial]
#[tokio::test]
async fn invoice_guia_serie_t() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                descripcion: "Item1".into(),
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
            Detalle {
                descripcion: "Item2".into(),
                cantidad: dec!(2),
                precio: Some(dec!(100)),
                ..Default::default()
            },
        ],
        guias: vec![Guia {
            tipo_comprobante: Catalog1::GuiaRemisionRemitente.code().into(),
            serie_numero: "T001-1".into(),
        }],
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/guia_serie_t.xml")).await;
}
