use crate::models::common::Cargo;
use crate::models::invoice::Invoice;

pub trait InvoiceCargosGetter {
    fn get_cargos(&mut self) -> &mut Vec<Cargo>;
}

impl InvoiceCargosGetter for Invoice {
    fn get_cargos(&mut self) -> &mut Vec<Cargo> {
        &mut self.cargos
    }
}
