use std::collections::HashMap;

use chrono::NaiveDate;

use crate::models::common::{Cliente, Firmante, Proveedor};

pub struct DebitNote {
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

    // Catalog10
    pub tipo_nota: Option<&'static str>,
    pub comprobante_afectado_serie_numero: &'static str,
    // Catalog1
    pub comprobante_afectado_tipo: Option<&'static str>,
    pub sustento_descripcion: &'static str,
}
