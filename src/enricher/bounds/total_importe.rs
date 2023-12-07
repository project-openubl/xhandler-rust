use crate::models::common::TotalImporte;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait TotalImporteGetter {
    fn get_total_importe(&self) -> &Option<TotalImporte>;
}

pub trait TotalImporteSetter {
    fn set_total_importe(&mut self, val: TotalImporte);
}

impl TotalImporteGetter for Invoice {
    fn get_total_importe(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteGetter for CreditNote {
    fn get_total_importe(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteGetter for DebitNote {
    fn get_total_importe(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteSetter for Invoice {
    fn set_total_importe(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}

impl TotalImporteSetter for CreditNote {
    fn set_total_importe(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}

impl TotalImporteSetter for DebitNote {
    fn set_total_importe(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}
