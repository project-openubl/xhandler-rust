use crate::models::common::Cargo;
use crate::models::invoice::Invoice;

pub trait InvoiceCargosGetter {
    fn cargos(&self) -> &[Cargo];
    fn cargos_mut(&mut self) -> &mut Vec<Cargo>;
}

impl InvoiceCargosGetter for Invoice {
    fn cargos(&self) -> &[Cargo] {
        &self.cargos
    }

    fn cargos_mut(&mut self) -> &mut Vec<Cargo> {
        &mut self.cargos
    }
}
