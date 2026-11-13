use chrono::NaiveDate;

use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::VoidedDocuments;

pub trait FechaEmisionGetter {
    fn fecha_emision(&self) -> &Option<NaiveDate>;
}

pub trait FechaEmisionSetter {
    fn set_fecha_emision(&mut self, val: NaiveDate);
}

//

impl FechaEmisionGetter for Invoice {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for CreditNote {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for DebitNote {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionSetter for Invoice {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for CreditNote {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for DebitNote {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionGetter for VoidedDocuments {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for SummaryDocuments {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionSetter for VoidedDocuments {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for SummaryDocuments {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionGetter for Perception {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionGetter for Retention {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionSetter for Perception {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionSetter for Retention {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}

impl FechaEmisionGetter for DespatchAdvice {
    fn fecha_emision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }
}

impl FechaEmisionSetter for DespatchAdvice {
    fn set_fecha_emision(&mut self, val: NaiveDate) {
        self.fecha_emision = Some(val);
    }
}
