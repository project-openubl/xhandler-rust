use chrono::NaiveDate;
use derive_builder::Builder;
use crate::content::models::common::{Firmante, Proveedor};

#[derive(Builder)]
pub struct DebitNote {
    #[builder(default)]
    pub moneda: Option<String>,

    #[builder(default)]
    pub fecha_emision: Option<NaiveDate>,

    pub proveedor: Proveedor,

    #[builder(default)]
    pub firmante: Option<Firmante>,

    //
    // leyendas: Option<Map<String, String>>,
    // tasa_ivap: Option<u8>,
    // tasa_icb: Option<u8>,
    //
    // fecha_vencimiento: Option<NaiveDate>,
}

