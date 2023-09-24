use crate::models::general::Detraccion;
use crate::models::invoice::Invoice;

pub trait DetraccionGetter {
    fn get_detraccion(&self) -> &Option<Detraccion>;
}

impl DetraccionGetter for Invoice {
    fn get_detraccion(&self) -> &Option<Detraccion> {
        &self.detraccion
    }
}
