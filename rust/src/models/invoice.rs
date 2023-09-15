use std::collections::HashMap;

use chrono::NaiveDate;

use crate::models::common::{
    Cliente, Detraccion, Direccion, Firmante, FormaDePago, Percepcion, Proveedor,
};

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

    /// Catalog1
    pub tipo_comprobante: Option<&'static str>,

    /// Catalog51
    pub tipo_operacion: Option<&'static str>,

    pub detraccion: Option<Detraccion>,
    pub percepcion: Option<Percepcion>,
    pub direccion_entrega: Option<Direccion>,
    pub forma_de_pago: Option<FormaDePago>,
}
