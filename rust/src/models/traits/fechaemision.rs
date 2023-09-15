use chrono::NaiveDate;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait FechaEmisionGetter {
    fn get_fechaemision(&self) -> &Option<NaiveDate>;
}

pub trait FechaEmisionSetter {
    fn set_fechaemision(&mut self, val: NaiveDate);
}

impl FechaEmisionGetter for Invoice {
    fn get_fechaemision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for CreditNote {
    fn get_fechaemision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for DebitNote {
    fn get_fechaemision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionSetter for Invoice {
    fn set_fechaemision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for CreditNote {
    fn set_fechaemision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for DebitNote {
    fn set_fechaemision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}
