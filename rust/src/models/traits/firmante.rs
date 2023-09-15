use crate::models::common::Firmante;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

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
