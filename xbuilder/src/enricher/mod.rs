use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;

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

/// Values used when the document does not set them. The tax rates default to
/// the current SUNAT rates; `fecha_emision` is required (usually today).
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Defaults {
    #[serde(default = "default_icb_tasa")]
    pub icb_tasa: Decimal,
    #[serde(default = "default_igv_tasa")]
    pub igv_tasa: Decimal,
    #[serde(default = "default_ivap_tasa")]
    pub ivap_tasa: Decimal,
    #[serde(deserialize_with = "crate::serde_date::deserialize")]
    pub fecha_emision: NaiveDate,
}

impl Defaults {
    /// Current SUNAT rates: ICB 0.20, IGV 18%, IVAP 4%.
    pub fn new(fecha_emision: NaiveDate) -> Self {
        Self {
            icb_tasa: default_icb_tasa(),
            igv_tasa: default_igv_tasa(),
            ivap_tasa: default_ivap_tasa(),
            fecha_emision,
        }
    }
}

fn default_icb_tasa() -> Decimal {
    Decimal::new(2, 1)
}

fn default_igv_tasa() -> Decimal {
    Decimal::new(18, 2)
}

fn default_ivap_tasa() -> Decimal {
    Decimal::new(4, 2)
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
