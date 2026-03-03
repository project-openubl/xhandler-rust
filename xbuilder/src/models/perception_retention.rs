use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{Cliente, Firmante, Proveedor};

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

/// Comprobante de Retencion
#[derive(Debug, Serialize, Default)]
pub struct Retention {
    pub serie: &'static str,
    pub numero: u32,
    pub fecha_emision: Option<NaiveDate>,
    pub moneda: Option<&'static str>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    /// Catalog23
    pub tipo_regimen: &'static str,
    pub tipo_regimen_porcentaje: Decimal,
    pub importe_total_retenido: Decimal,
    pub importe_total_pagado: Decimal,
    pub observacion: Option<&'static str>,
    pub operacion: Option<PercepcionRetencionOperacion>,
    /// Computed during enrichment: "{serie}-{numero}"
    pub documento_id: Option<String>,
}

/// Operacion de percepcion o retencion
#[derive(Clone, Debug, Serialize)]
pub struct PercepcionRetencionOperacion {
    pub numero_operacion: u32,
    pub fecha_operacion: NaiveDate,
    pub importe_operacion: Decimal,
    pub comprobante: PercepcionRetencionComprobanteAfectado,
    pub tipo_cambio: Option<TipoCambio>,
}

/// Comprobante afectado por la percepcion o retencion
#[derive(Clone, Debug, Serialize)]
pub struct PercepcionRetencionComprobanteAfectado {
    pub tipo_comprobante: &'static str,
    pub serie_numero: &'static str,
    pub fecha_emision: NaiveDate,
    pub importe_total: Decimal,
    pub moneda: Option<&'static str>,
}

/// Tipo de cambio
#[derive(Clone, Debug, Serialize)]
pub struct TipoCambio {
    pub valor: Decimal,
    pub fecha: NaiveDate,
}
