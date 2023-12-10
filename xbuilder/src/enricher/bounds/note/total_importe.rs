use crate::models::common::TotalImporteNote;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;

pub trait NoteTotalImporteGetter {
    fn get_total_importe(&self) -> &Option<TotalImporteNote>;
}

pub trait NoteTotalImporteSetter {
    fn set_total_importe(&mut self, val: TotalImporteNote);
}

impl NoteTotalImporteGetter for CreditNote {
    fn get_total_importe(&self) -> &Option<TotalImporteNote> {
        &self.total_importe
    }
}

impl NoteTotalImporteGetter for DebitNote {
    fn get_total_importe(&self) -> &Option<TotalImporteNote> {
        &self.total_importe
    }
}

impl NoteTotalImporteSetter for CreditNote {
    fn set_total_importe(&mut self, val: TotalImporteNote) {
        self.total_importe = Some(val);
    }
}

impl NoteTotalImporteSetter for DebitNote {
    fn set_total_importe(&mut self, val: TotalImporteNote) {
        self.total_importe = Some(val);
    }
}
