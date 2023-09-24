use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait IGVTasaGetter {
    fn get_igv_tasa(&self) -> &Option<f32>;
}

pub trait IGVTasaSetter {
    fn set_igv_tasa(&mut self, val: f32);
}

impl IGVTasaGetter for Invoice {
    fn get_igv_tasa(&self) -> &Option<f32> {
        &self.igv_tasa
    }
}

impl IGVTasaGetter for CreditNote {
    fn get_igv_tasa(&self) -> &Option<f32> {
        &self.igv_tasa
    }
}

impl IGVTasaGetter for DebitNote {
    fn get_igv_tasa(&self) -> &Option<f32> {
        &self.igv_tasa
    }
}

impl IGVTasaSetter for Invoice {
    fn set_igv_tasa(&mut self, val: f32) {
        self.igv_tasa = Some(val);
    }
}

impl IGVTasaSetter for CreditNote {
    fn set_igv_tasa(&mut self, val: f32) {
        self.igv_tasa = Some(val);
    }
}

impl IGVTasaSetter for DebitNote {
    fn set_igv_tasa(&mut self, val: f32) {
        self.igv_tasa = Some(val);
    }
}
