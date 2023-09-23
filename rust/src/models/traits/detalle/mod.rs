use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::general::Detalle;
use crate::models::invoice::Invoice;

pub mod icbtasa;
pub mod unidadmedida;

pub trait DetallesGetter {
    fn get_detalles(&mut self) -> &mut Vec<Detalle>;
}

impl DetallesGetter for Invoice {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}

impl DetallesGetter for CreditNote {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}

impl DetallesGetter for DebitNote {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}
