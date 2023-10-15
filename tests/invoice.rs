use xbuilder::models::general::Detalle;
use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::detalle_base;
use crate::common::invoice_base;

mod common;

#[test]
fn custom_unidad_medida() {
    let mut invoice = Invoice {
        detalles: vec![
            Detalle {
                precio: Some(100f64),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item1", 10f64)
            },
            Detalle {
                precio: Some(100f64),
                unidad_medida: Some("KGM"),
                ..detalle_base("Item2", 10f64)
            },
        ],
        ..invoice_base()
    };

    assert_invoice(
        &mut invoice,
        "tests/resources/e2e/renderer/invoice/InvoiceTest/customUnidadMedida.xml",
    );
}
