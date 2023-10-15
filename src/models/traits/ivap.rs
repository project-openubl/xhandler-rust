use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait IVAPTasaGetter {
    fn get_ivap_tasa(&self) -> &Option<f64>;
}

pub trait IVAPTasaSetter {
    fn set_ivap_tasa(&mut self, val: f64);
}

impl IVAPTasaGetter for Invoice {
    fn get_ivap_tasa(&self) -> &Option<f64> {
        &self.ivap_tasa
    }
}

impl IVAPTasaGetter for CreditNote {
    fn get_ivap_tasa(&self) -> &Option<f64> {
        &self.ivap_tasa
    }
}

impl IVAPTasaGetter for DebitNote {
    fn get_ivap_tasa(&self) -> &Option<f64> {
        &self.ivap_tasa
    }
}

impl IVAPTasaSetter for Invoice {
    fn set_ivap_tasa(&mut self, val: f64) {
        self.ivap_tasa = Some(val);
    }
}

impl IVAPTasaSetter for CreditNote {
    fn set_ivap_tasa(&mut self, val: f64) {
        self.ivap_tasa = Some(val);
    }
}

impl IVAPTasaSetter for DebitNote {
    fn set_ivap_tasa(&mut self, val: f64) {
        self.ivap_tasa = Some(val);
    }
}
