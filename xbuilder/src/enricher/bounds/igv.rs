use rust_decimal::Decimal;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait IgvTasaGetter {
    fn get_igv_tasa(&self) -> &Option<Decimal>;
}

pub trait IgvTasaSetter {
    fn set_igv_tasa(&mut self, val: Decimal);
}

impl IgvTasaGetter for Invoice {
    fn get_igv_tasa(&self) -> &Option<Decimal> {
        &self.igv_tasa
    }
}

impl IgvTasaGetter for CreditNote {
    fn get_igv_tasa(&self) -> &Option<Decimal> {
        &self.igv_tasa
    }
}

impl IgvTasaGetter for DebitNote {
    fn get_igv_tasa(&self) -> &Option<Decimal> {
        &self.igv_tasa
    }
}

impl IgvTasaSetter for Invoice {
    fn set_igv_tasa(&mut self, val: Decimal) {
        self.igv_tasa = Some(val);
    }
}

impl IgvTasaSetter for CreditNote {
    fn set_igv_tasa(&mut self, val: Decimal) {
        self.igv_tasa = Some(val);
    }
}

impl IgvTasaSetter for DebitNote {
    fn set_igv_tasa(&mut self, val: Decimal) {
        self.igv_tasa = Some(val);
    }
}
