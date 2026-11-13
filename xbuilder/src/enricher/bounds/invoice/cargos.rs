use crate::models::common::Cargo;
use crate::models::invoice::Invoice;

pub trait InvoiceCargosGetter {
    fn get_cargos(&self) -> &Vec<Cargo>;
}

impl InvoiceCargosGetter for Invoice {
    fn get_cargos(&self) -> &Vec<Cargo> {
        &self.cargos
    }
}
