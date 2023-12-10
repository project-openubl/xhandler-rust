use crate::models::common::TotalImporteInvoice;
use crate::models::invoice::Invoice;

pub trait InvoiceTotalImporteGetter {
    fn get_total_importe(&self) -> &Option<TotalImporteInvoice>;
}

pub trait InvoiceTotalImporteSetter {
    fn set_total_importe(&mut self, val: TotalImporteInvoice);
}

impl InvoiceTotalImporteGetter for Invoice {
    fn get_total_importe(&self) -> &Option<TotalImporteInvoice> {
        &self.total_importe
    }
}

impl InvoiceTotalImporteSetter for Invoice {
    fn set_total_importe(&mut self, val: TotalImporteInvoice) {
        self.total_importe = Some(val);
    }
}
