use crate::models::general::Detraccion;
use crate::models::invoice::Invoice;

pub trait InvoiceDetraccionGetter {
    fn get_detraccion(&mut self) -> &mut Option<Detraccion>;
}

impl InvoiceDetraccionGetter for Invoice {
    fn get_detraccion(&mut self) -> &mut Option<Detraccion> {
        &mut self.detraccion
    }
}
