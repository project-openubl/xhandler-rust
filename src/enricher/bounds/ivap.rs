use rust_decimal::Decimal;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait IvapTasaGetter {
    fn get_ivap_tasa(&self) -> &Option<Decimal>;
}

pub trait IvapTasaSetter {
    fn set_ivap_tasa(&mut self, val: Decimal);
}

impl IvapTasaGetter for Invoice {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IvapTasaGetter for CreditNote {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IvapTasaGetter for DebitNote {
    fn get_ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IvapTasaSetter for Invoice {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}

impl IvapTasaSetter for CreditNote {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}

impl IvapTasaSetter for DebitNote {
    fn set_ivap_tasa(&mut self, val: Decimal) {
        self.ivap_tasa = Some(val);
    }
}
