use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait MonedaGetter {
    fn get_moneda(&self) -> &Option<String>;
}

pub trait MonedaSetter {
    fn set_moneda(&mut self, val: &str);
}

impl MonedaGetter for Invoice {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaGetter for CreditNote {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaGetter for DebitNote {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaSetter for Invoice {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}

impl MonedaSetter for CreditNote {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}

impl MonedaSetter for DebitNote {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}
