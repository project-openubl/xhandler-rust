use chrono::NaiveDate;

use xbuilder::prelude::*;

use crate::common::assert_invoice;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceFechaVencimientoTest";

#[test]
fn invoice_custom_moneda() {
    let mut invoice = Invoice {
        fecha_vencimiento: NaiveDate::from_ymd_opt(2022, 1, 1),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/conFechaVencimiento.xml"));
}
