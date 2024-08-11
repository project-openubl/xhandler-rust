use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDocumentoRelacionadoTest";

#[serial_test::serial]
#[tokio::test]
async fn invoice_documento_relacionado_y_orden_de_compra() {
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
        documentos_relacionados: vec![DocumentoRelacionado {
            serie_numero: "B111-1",
            tipo_documento: Catalog12::DeclaracionSimplificadaDeImportacion.code(),
        }],
        orden_de_compra: Some("123456"),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/documentoRelacionado.xml")).await;
}
