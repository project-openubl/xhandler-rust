use crate::models::general::Detraccion;
use crate::models::invoice::Invoice;

pub trait InvoiceDetraccionGetter {
    fn get_detraccion(&self) -> &Option<Detraccion>;
}

impl InvoiceDetraccionGetter for Invoice {
    fn get_detraccion(&self) -> &Option<Detraccion> {
        &self.detraccion
    }
}
