//! A nota de crédito type 13 carries the corrected `forma_pago`, filled by
//! the same rules as an invoice's (its total, its type) but never defaulted.

use crate::xbuilder::enricher::bounds::invoice::forma_pago::{
    InvoiceFormaPagoGetter, InvoiceFormaPagoSetter,
};
use crate::xbuilder::models::common::FormaPago;
use crate::xbuilder::models::credit_note::CreditNote;

impl InvoiceFormaPagoGetter for CreditNote {
    fn forma_pago(&self) -> &Option<FormaPago> {
        &self.forma_pago
    }
}

impl InvoiceFormaPagoSetter for CreditNote {
    fn set_forma_pago(&mut self, val: FormaPago) {
        self.forma_pago = Some(val);
    }
}
