use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;

pub trait MonedaGetter {
    fn moneda(&self) -> &Option<String>;
}

pub trait MonedaSetter {
    fn set_moneda(&mut self, val: &str);
}

impl MonedaGetter for Invoice {
    fn moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaGetter for CreditNote {
    fn moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaGetter for DebitNote {
    fn moneda(&self) -> &Option<String> {
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

impl MonedaGetter for Perception {
    fn moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaSetter for Perception {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}

impl MonedaGetter for Retention {
    fn moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaSetter for Retention {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}

impl MonedaGetter for SummaryDocuments {
    fn moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl MonedaSetter for SummaryDocuments {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}
