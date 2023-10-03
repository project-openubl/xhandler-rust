use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::general::TotalImporte;
use crate::models::invoice::Invoice;

pub trait TotalImporteGetter {
    fn get_totalimporte(&self) -> &Option<TotalImporte>;
}

pub trait TotalImporteSetter {
    fn set_totalimporte(&mut self, val: TotalImporte);
}

impl TotalImporteGetter for Invoice {
    fn get_totalimporte(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteGetter for CreditNote {
    fn get_totalimporte(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteGetter for DebitNote {
    fn get_totalimporte(&self) -> &Option<TotalImporte> {
        &self.total_importe
    }
}

impl TotalImporteSetter for Invoice {
    fn set_totalimporte(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}

impl TotalImporteSetter for CreditNote {
    fn set_totalimporte(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}

impl TotalImporteSetter for DebitNote {
    fn set_totalimporte(&mut self, val: TotalImporte) {
        self.total_importe = Some(val);
    }
}
