use std::collections::HashMap;

use chrono::NaiveDate;

use crate::models::common::{Cliente, Detraccion, Firmante, Proveedor};

pub struct Invoice {
    pub leyendas: HashMap<&'static str, &'static str>,

    pub serie_numero: &'static str,
    pub moneda: Option<&'static str>,
    pub fecha_emision: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    pub icb_tasa: Option<f32>,
    pub igv_tasa: Option<f32>,
    pub ivap_tasa: Option<f32>,

    pub detraccion: Option<Detraccion>,
}
