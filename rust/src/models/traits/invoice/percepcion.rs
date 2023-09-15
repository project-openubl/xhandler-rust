use crate::models::common::Percepcion;
use crate::models::invoice::Invoice;

pub trait PercepcionGetter {
    fn get_percepcion(&self) -> &Option<Percepcion>;
}

impl PercepcionGetter for Invoice {
    fn get_percepcion(&self) -> &Option<Percepcion> {
        &self.percepcion
    }
}
