use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::enricher::fill::FillTrait;
use crate::enricher::process::ProcessTrait;
use crate::enricher::summary::SummaryTrait;
use crate::models::invoice::Invoice;
use crate::prelude::{CreditNote, DebitNote};

pub mod bounds;
pub mod fill;
pub mod process;
pub mod rules;
pub mod summary;

pub struct Defaults {
    pub icb_tasa: Decimal,
    pub igv_tasa: Decimal,
    pub ivap_tasa: Decimal,
    pub date: NaiveDate,
}

pub trait EnrichTrait {
    fn enrich(&mut self, defaults: &Defaults);
}

impl EnrichTrait for Invoice {
    fn enrich(&mut self, defaults: &Defaults) {
        FillTrait::fill(self, defaults);
        ProcessTrait::process(self);
        SummaryTrait::summary(self);
    }
}

impl EnrichTrait for CreditNote {
    fn enrich(&mut self, defaults: &Defaults) {
        FillTrait::fill(self, defaults);
        ProcessTrait::process(self);
        SummaryTrait::summary(self);
    }
}

impl EnrichTrait for DebitNote {
    fn enrich(&mut self, defaults: &Defaults) {
        FillTrait::fill(self, defaults);
        ProcessTrait::process(self);
        SummaryTrait::summary(self);
    }
}
