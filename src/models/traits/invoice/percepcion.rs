use crate::models::general::Percepcion;
use crate::models::invoice::Invoice;

pub trait InvoicePercepcionGetter {
    fn get_percepcion(&self) -> &Option<Percepcion>;
}

impl InvoicePercepcionGetter for Invoice {
    fn get_percepcion(&self) -> &Option<Percepcion> {
        &self.percepcion
    }
}
