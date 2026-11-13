use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::enricher::fill::Fill;
use crate::enricher::process::Process;
use crate::enricher::summary::Summary;
use crate::enricher::validate::Validate;
use crate::models::voided_documents::Reversion;

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
    pub fecha_emision: NaiveDate,
}

pub trait Enrich {
    /// Fills defaults and computes the amounts of the document. Fails when a
    /// catalog code (e.g. `igv_tipo`) is not valid.
    fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()>;
}

impl<T: Fill + Process + Summary + Validate> Enrich for T {
    fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()> {
        Fill::fill(self, defaults);
        Process::process(self);
        Summary::summary(self);
        self.validate()
    }
}

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
