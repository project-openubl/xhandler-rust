use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{Cliente, Firmante, PercepcionRetencionOperacion, Proveedor};

/// Comprobante de Percepcion
#[derive(Debug, Serialize, Default)]
pub struct Perception {
    pub serie: &'static str,
    pub numero: u32,
    pub fecha_emision: Option<NaiveDate>,
    pub moneda: Option<&'static str>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    /// Catalog22
    pub tipo_regimen: &'static str,
    pub tipo_regimen_porcentaje: Decimal,
    pub importe_total_percibido: Decimal,
    pub importe_total_cobrado: Decimal,
    pub observacion: Option<&'static str>,
    pub operacion: Option<PercepcionRetencionOperacion>,
    /// Computed during enrichment: "{serie}-{numero}"
    pub documento_id: Option<String>,
}
