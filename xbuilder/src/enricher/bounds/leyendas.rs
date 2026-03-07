use std::collections::HashMap;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait LeyendasGetter {
    fn get_leyendas(&self) -> &HashMap<String, String>;
}

pub trait LeyendasSetter {
    fn insert_leyendas(&mut self, k: &str, v: &str);
}

impl LeyendasGetter for Invoice {
    fn get_leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasGetter for CreditNote {
    fn get_leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasGetter for DebitNote {
    fn get_leyendas(&self) -> &HashMap<String, String> {
        &self.leyendas
    }
}

impl LeyendasSetter for Invoice {
    fn insert_leyendas(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}

impl LeyendasSetter for CreditNote {
    fn insert_leyendas(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}

impl LeyendasSetter for DebitNote {
    fn insert_leyendas(&mut self, k: &str, v: &str) {
        self.leyendas.insert(k.to_string(), v.to_string());
    }
}
