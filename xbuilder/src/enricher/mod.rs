use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::enricher::fill::Fill;
use crate::enricher::process::Process;
use crate::enricher::summary::Summary;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

mod bounds;
mod fill;
mod process;
mod rules;
mod summary;

pub struct Defaults {
    pub icb_tasa: Decimal,
    pub igv_tasa: Decimal,
    pub ivap_tasa: Decimal,
    pub date: NaiveDate,
}

pub trait Enrich {
    fn enrich(&mut self, defaults: &Defaults);
}

impl Enrich for Invoice {
    fn enrich(&mut self, defaults: &Defaults) {
        Fill::fill(self, defaults);
        Process::process(self);
        Summary::summary(self);
    }
}

impl Enrich for CreditNote {
    fn enrich(&mut self, defaults: &Defaults) {
        Fill::fill(self, defaults);
        Process::process(self);
        Summary::summary(self);
    }
}

impl Enrich for DebitNote {
    fn enrich(&mut self, defaults: &Defaults) {
        Fill::fill(self, defaults);
        Process::process(self);
        Summary::summary(self);
    }
}
