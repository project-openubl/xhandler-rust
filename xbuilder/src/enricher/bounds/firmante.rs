use crate::models::common::Firmante;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception::Perception;
use crate::models::retention::Retention;
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::VoidedDocuments;

pub trait FirmanteGetter {
    fn get_firmante(&self) -> &Option<Firmante>;
}

pub trait FirmanteSetter {
    fn set_firmante(&mut self, val: Firmante);
}

impl FirmanteGetter for Invoice {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for CreditNote {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for DebitNote {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteSetter for Invoice {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteSetter for CreditNote {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteSetter for DebitNote {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteGetter for VoidedDocuments {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for SummaryDocuments {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteSetter for VoidedDocuments {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteSetter for SummaryDocuments {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteGetter for Perception {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for Retention {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteSetter for Perception {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteSetter for Retention {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}

impl FirmanteGetter for DespatchAdvice {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteSetter for DespatchAdvice {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}
