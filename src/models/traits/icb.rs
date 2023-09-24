use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait ICBTasaGetter {
    fn get_icb_tasa(&self) -> &Option<f32>;
}

pub trait ICBTasaSetter {
    fn set_icb_tasa(&mut self, val: f32);
}

impl ICBTasaGetter for Invoice {
    fn get_icb_tasa(&self) -> &Option<f32> {
        &self.icb_tasa
    }
}

impl ICBTasaGetter for CreditNote {
    fn get_icb_tasa(&self) -> &Option<f32> {
        &self.icb_tasa
    }
}

impl ICBTasaGetter for DebitNote {
    fn get_icb_tasa(&self) -> &Option<f32> {
        &self.icb_tasa
    }
}

impl ICBTasaSetter for Invoice {
    fn set_icb_tasa(&mut self, val: f32) {
        self.icb_tasa = Some(val);
    }
}

impl ICBTasaSetter for CreditNote {
    fn set_icb_tasa(&mut self, val: f32) {
        self.icb_tasa = Some(val);
    }
}

impl ICBTasaSetter for DebitNote {
    fn set_icb_tasa(&mut self, val: f32) {
        self.icb_tasa = Some(val);
    }
}
