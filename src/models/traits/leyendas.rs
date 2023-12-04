use std::collections::HashMap;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait LeyendasGetter {
    fn get_leyendas(&self) -> &HashMap<&str, &str>;
}

pub trait LeyendasSetter {
    fn insert_leyendas(&mut self, k: &'static str, v: &'static str);
}

impl LeyendasGetter for Invoice {
    fn get_leyendas(&self) -> &HashMap<&str, &str> {
        &self.leyendas
    }
}

impl LeyendasGetter for CreditNote {
    fn get_leyendas(&self) -> &HashMap<&str, &str> {
        &self.leyendas
    }
}

impl LeyendasGetter for DebitNote {
    fn get_leyendas(&self) -> &HashMap<&str, &str> {
        &self.leyendas
    }
}

impl LeyendasSetter for Invoice {
    fn insert_leyendas(&mut self, k: &'static str, v: &'static str) {
        self.leyendas.insert(k, v);
    }
}

impl LeyendasSetter for CreditNote {
    fn insert_leyendas(&mut self, k: &'static str, v: &'static str) {
        self.leyendas.insert(k, v);
    }
}

impl LeyendasSetter for DebitNote {
    fn insert_leyendas(&mut self, k: &'static str, v: &'static str) {
        self.leyendas.insert(k, v);
    }
}
