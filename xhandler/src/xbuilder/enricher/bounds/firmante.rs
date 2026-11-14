use crate::xbuilder::models::common::Firmante;
use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::despatch_advice::DespatchAdvice;
use crate::xbuilder::models::invoice::Invoice;
use crate::xbuilder::models::perception_retention::{Perception, Retention};
use crate::xbuilder::models::summary_documents::SummaryDocuments;
use crate::xbuilder::models::voided_documents::VoidedDocuments;

pub trait FirmanteGetter {
    fn firmante(&self) -> &Option<Firmante>;
}

pub trait FirmanteSetter {
    fn set_firmante(&mut self, val: Firmante);
}

impl FirmanteGetter for Invoice {
    fn firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for CreditNote {
    fn firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for DebitNote {
    fn firmante(&self) -> &Option<Firmante> {
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
    fn firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for SummaryDocuments {
    fn firmante(&self) -> &Option<Firmante> {
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
    fn firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteGetter for Retention {
    fn firmante(&self) -> &Option<Firmante> {
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
    fn firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }
}

impl FirmanteSetter for DespatchAdvice {
    fn set_firmante(&mut self, val: Firmante) {
        self.firmante = Some(val);
    }
}
