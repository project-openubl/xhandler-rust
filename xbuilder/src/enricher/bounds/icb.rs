use rust_decimal::Decimal;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait IcbTasaGetter {
    fn get_icb_tasa(&self) -> &Option<Decimal>;
}

pub trait IcbTasaSetter {
    fn set_icb_tasa(&mut self, val: Decimal);
}

impl IcbTasaGetter for Invoice {
    fn get_icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl IcbTasaGetter for CreditNote {
    fn get_icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl IcbTasaGetter for DebitNote {
    fn get_icb_tasa(&self) -> &Option<Decimal> {
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
