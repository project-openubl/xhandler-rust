use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::enricher::fill::Fill;
use crate::enricher::process::Process;
use crate::enricher::summary::Summary;
use crate::enricher::validate::Validate;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::{Reversion, VoidedDocuments};

mod bounds;
mod fill;
mod process;
mod rules;
mod summary;
mod validate;

pub struct Defaults {
    pub icb_tasa: Decimal,
    pub igv_tasa: Decimal,
    pub ivap_tasa: Decimal,
    pub date: NaiveDate,
}

pub trait Enrich {
    /// Fills defaults and computes the amounts of the document. Fails when a
    /// catalog code (e.g. `igv_tipo`) is not valid.
    fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()>;
}

fn enrich_document<T: Fill + Process + Summary + Validate>(
    document: &mut T,
    defaults: &Defaults,
) -> anyhow::Result<()> {
    Fill::fill(document, defaults);
    Process::process(document);
    Summary::summary(document);
    document.validate()
}

macro_rules! impl_enrich {
    ($($document:ty),*) => {
        $(impl Enrich for $document {
            fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()> {
                enrich_document(self, defaults)
            }
        })*
    };
}

impl_enrich!(
    Invoice,
    CreditNote,
    DebitNote,
    VoidedDocuments,
    SummaryDocuments,
    Perception,
    Retention,
    DespatchAdvice
);

impl Enrich for Reversion {
    fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()> {
        if self.0.documento_id.is_some() {
            return self.0.enrich(defaults);
        }

        self.0.enrich(defaults)?;
        if let Some(id) = self.0.documento_id.as_mut() {
            if let Some(rest) = id.strip_prefix("RA") {
                *id = format!("RR{rest}");
            }
        }
        Ok(())
    }
}
