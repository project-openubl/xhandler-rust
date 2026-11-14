use rust_decimal::Decimal;

use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::invoice::Invoice;

pub trait IvapTasaGetter {
    fn ivap_tasa(&self) -> &Option<Decimal>;
}

pub trait IvapTasaSetter {
    fn set_ivap_tasa(&mut self, val: Decimal);
}

impl IvapTasaGetter for Invoice {
    fn ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IvapTasaGetter for CreditNote {
    fn ivap_tasa(&self) -> &Option<Decimal> {
        &self.ivap_tasa
    }
}

impl IvapTasaGetter for DebitNote {
    fn ivap_tasa(&self) -> &Option<Decimal> {
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
