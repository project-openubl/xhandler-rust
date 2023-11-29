use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;
use rust_decimal::Decimal;

pub trait IVAPTasaGetter {
    fn get_ivap_tasa(&self) -> &Option<Decimal>;
}

pub trait IVAPTasaSetter {
    fn set_ivap_tasa(&mut self, val: Decimal);
}

impl IVAPTasaGetter for Invoice {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IVAPTasaGetter for CreditNote {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IVAPTasaGetter for DebitNote {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IVAPTasaSetter for Invoice {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}

impl IVAPTasaSetter for CreditNote {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}

impl IVAPTasaSetter for DebitNote {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}
