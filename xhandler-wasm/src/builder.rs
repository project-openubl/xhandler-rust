use serde::Deserialize;
use serde_json::Value;
use xbuilder::prelude::*;

use crate::ops::{parse_input, Error, ErrorKind, OpResult};

#[derive(Deserialize)]
struct Input {
    document: Document,
    defaults: Defaults,
}

fn enriched(input: Value) -> Result<Document, Error> {
    let Input {
        mut document,
        defaults,
    } = parse_input(input)?;
    document
        .enrich(&defaults)
        .map_err(|e| Error::new(ErrorKind::Validation, e))?;
    Ok(document)
}

/// `{document, defaults}` -> the enriched document, `{kind, spec}`.
pub fn enrich(input: Value) -> OpResult {
    let document = enriched(input)?;
    serde_json::to_value(document).map_err(|e| Error::new(ErrorKind::Render, e))
}

/// `{document, defaults}` -> the XML of the enriched document.
pub fn build(input: Value) -> OpResult {
    let xml = enriched(input)?
        .render()
        .map_err(|e| Error::new(ErrorKind::Render, e))?;
    Ok(xml.into())
}
