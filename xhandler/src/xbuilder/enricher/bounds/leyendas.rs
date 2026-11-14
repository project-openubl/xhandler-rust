use std::collections::HashMap;

use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::invoice::Invoice;

pub trait LeyendasGetter {
    fn leyendas(&self) -> &HashMap<String, String>;
}

pub trait LeyendasSetter {
    fn insert_leyenda(&mut self, k: &str, v: &str);
}

impl LeyendasGetter for Invoice {
    fn leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasGetter for CreditNote {
    fn leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasGetter for DebitNote {
    fn leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasSetter for Invoice {
    fn insert_leyenda(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}

impl LeyendasSetter for CreditNote {
    fn insert_leyenda(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}

impl LeyendasSetter for DebitNote {
    fn insert_leyenda(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}
