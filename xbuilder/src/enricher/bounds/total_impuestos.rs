use crate::models::common::TotalImpuestos;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait TotalImpuestosGetter {
    fn total_impuestos(&self) -> &Option<TotalImpuestos>;
}

pub trait TotalImpuestosSetter {
    fn set_total_impuestos(&mut self, val: TotalImpuestos);
}

impl TotalImpuestosGetter for Invoice {
    fn total_impuestos(&self) -> &Option<TotalImpuestos> {
        &self.total_impuestos
    }
}

impl TotalImpuestosGetter for CreditNote {
    fn total_impuestos(&self) -> &Option<TotalImpuestos> {
        &self.total_impuestos
    }
}

impl TotalImpuestosGetter for DebitNote {
    fn total_impuestos(&self) -> &Option<TotalImpuestos> {
        &self.total_impuestos
    }
}

impl TotalImpuestosSetter for Invoice {
    fn set_total_impuestos(&mut self, val: TotalImpuestos) {
        self.total_impuestos = Some(val);
    }
}

impl TotalImpuestosSetter for CreditNote {
    fn set_total_impuestos(&mut self, val: TotalImpuestos) {
        self.total_impuestos = Some(val);
    }
}

impl TotalImpuestosSetter for DebitNote {
    fn set_total_impuestos(&mut self, val: TotalImpuestos) {
        self.total_impuestos = Some(val);
    }
}
