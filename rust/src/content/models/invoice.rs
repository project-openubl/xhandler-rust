use chrono::NaiveDate;
use derive_builder::Builder;
use crate::content::models::common::{Firmante, Proveedor};

#[derive(Builder)]
pub struct Invoice {
    #[builder(default)]
    pub moneda: Option<String>,

    #[builder(default)]
    pub fecha_emision: Option<NaiveDate>,

    pub proveedor: Proveedor,

    #[builder(default)]
    pub firmante: Option<Firmante>,
}

