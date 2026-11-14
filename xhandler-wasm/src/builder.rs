use chrono::{FixedOffset, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value;
use xbuilder::prelude::*;

use crate::ops::{parse_input, Error, ErrorKind, OpResult};

#[derive(Deserialize)]
struct Input {
    document: Document,
    #[serde(default)]
    defaults: DefaultsInput,
}

/// Values used when the document does not set them; every field is optional.
#[derive(Default, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "Defaults"))]
pub struct DefaultsInput {
    /// Today in Peru by default
    #[serde(
        default,
        deserialize_with = "xbuilder::serde_date::option::deserialize"
    )]
    pub fecha_emision: Option<NaiveDate>,
    /// The current SUNAT rate by default
    pub icb_tasa: Option<Decimal>,
    /// The current SUNAT rate by default
    pub igv_tasa: Option<Decimal>,
    /// The current SUNAT rate by default
    pub ivap_tasa: Option<Decimal>,
}

impl DefaultsInput {
    fn into_defaults(self) -> Defaults {
        let mut defaults = Defaults::new(self.fecha_emision.unwrap_or_else(today_in_peru));
        defaults.icb_tasa = self.icb_tasa.unwrap_or(defaults.icb_tasa);
        defaults.igv_tasa = self.igv_tasa.unwrap_or(defaults.igv_tasa);
        defaults.ivap_tasa = self.ivap_tasa.unwrap_or(defaults.ivap_tasa);
        defaults
    }
}

/// Peru is UTC-5 all year long
fn today_in_peru() -> NaiveDate {
    let peru = FixedOffset::west_opt(5 * 3600).expect("valid offset");
    Utc::now().with_timezone(&peru).date_naive()
}

fn enriched(input: Value) -> Result<Document, Error> {
    let Input {
        mut document,
        defaults,
    } = parse_input(input)?;
    document
        .enrich(&defaults.into_defaults())
        .map_err(|e| Error::new(ErrorKind::Validation, e))?;
    Ok(document)
}

/// `{document, defaults?}` -> the spec of the enriched document.
pub fn enrich(input: Value) -> OpResult {
    let mut document = serde_json::to_value(enriched(input)?).expect("documents are serializable");
    Ok(document["spec"].take())
}

/// `{document, defaults?}` -> the XML of the enriched document.
pub fn build(input: Value) -> OpResult {
    let xml = enriched(input)?
        .render()
        .map_err(|e| Error::new(ErrorKind::Render, e))?;
    Ok(xml.into())
}
