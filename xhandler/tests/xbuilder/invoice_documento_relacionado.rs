use rust_decimal_macros::dec;

use xhandler::xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/invoice_documento_relacionado";

#[serial_test::serial]
#[tokio::test]
async fn invoice_documento_relacionado_y_orden_compra() {
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
        documentos_relacionados: vec![DocumentoRelacionado {
            serie_numero: "B111-1".into(),
            tipo_documento: Catalog12::DeclaracionSimplificadaDeImportacion
                .code()
                .into(),
        }],
        orden_compra: Some("123456".into()),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/documento_relacionado.xml")).await;
}
