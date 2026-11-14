use rust_decimal::Decimal;

use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::invoice::Invoice;

pub trait IcbTasaGetter {
    fn icb_tasa(&self) -> &Option<Decimal>;
}

pub trait IcbTasaSetter {
    fn set_icb_tasa(&mut self, val: Decimal);
}

impl IcbTasaGetter for Invoice {
    fn icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl IcbTasaGetter for CreditNote {
    fn icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl IcbTasaGetter for DebitNote {
    fn icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl IcbTasaSetter for Invoice {
    fn set_icb_tasa(&mut self, val: Decimal) {
        self.icb_tasa = Some(val);
    }
}

impl IcbTasaSetter for CreditNote {
    fn set_icb_tasa(&mut self, val: Decimal) {
        self.icb_tasa = Some(val);
    }
}

impl IcbTasaSetter for DebitNote {
    fn set_icb_tasa(&mut self, val: Decimal) {
        self.icb_tasa = Some(val);
    }
}
