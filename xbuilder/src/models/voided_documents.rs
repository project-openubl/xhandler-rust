use chrono::NaiveDate;
use serde::Serialize;

use crate::models::common::{Firmante, Proveedor};

/// Comunicacion de Baja
#[derive(Debug, Serialize, Default)]
pub struct VoidedDocuments {
    pub numero: u32,
    pub fecha_emision: Option<NaiveDate>,
    pub fecha_emision_comprobantes: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub firmante: Option<Firmante>,
    pub comprobantes: Vec<VoidedDocumentsItem>,
    /// Computed during enrichment: "RA-YYYYMMDD-{numero}"
    pub documento_id: Option<String>,
}

/// Detalle de un comprobante dado de baja
#[derive(Clone, Debug, Serialize, Default)]
pub struct VoidedDocumentsItem {
    /// Catalog1: auto-filled from serie prefix (F->"01", B->"03")
    pub tipo_comprobante: Option<&'static str>,
    pub serie: &'static str,
    pub numero: u32,
    pub descripcion_sustento: &'static str,
}
