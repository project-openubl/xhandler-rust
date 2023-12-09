use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::invoice_base;
use crate::common::{assert_invoice, detalle_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceDocumentoRelacionadoTest";

#[test]
fn invoice_documento_relacionado_y_orden_de_compra() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item1", dec!(2))
            },
            Detalle {
                precio: Some(dec!(100)),
                ..detalle_base("Item2", dec!(2))
            },
        ],
        documentos_relacionados: vec![DocumentoRelacionado {
            serie_numero: "B111-1",
            tipo_documento: Catalog12::DeclaracionSimplificadaDeImportacion.code(),
        }],
        orden_de_compra: Some("123456"),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/documentoRelacionado.xml"));
}
