use crate::content::models::credit_note::CreditNote;
use crate::content::models::debit_note::DebitNote;
use crate::content::models::invoice::Invoice;

pub trait MonedaRule {
    fn enrich_moneda(&mut self) -> bool;
}

pub trait Moneda {
    fn get_moneda(&self) -> &Option<String>;
    fn set_moneda(&mut self, val: Option<String>);
}

impl<T> MonedaRule for T
    where T: Moneda, {
    fn enrich_moneda(&mut self) -> bool {
        match &self.get_moneda() {
            Some(..) => false,
            None => {
                self.set_moneda(Some("PEN".to_string()));
                true
            }
        }
    }
}

impl Moneda for Invoice {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }

    fn set_moneda(&mut self, val: Option<String>) {
        self.moneda = val;
    }
}

impl Moneda for CreditNote {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }

    fn set_moneda(&mut self, val: Option<String>) {
        self.moneda = val;
    }
}

impl Moneda for DebitNote {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }

    fn set_moneda(&mut self, val: Option<String>) {
        self.moneda = val;
    }
}