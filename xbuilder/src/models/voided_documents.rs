use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::common::{Firmante, Proveedor};

/// Comunicacion de Baja
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct VoidedDocuments {
    pub numero: u32,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision: Option<NaiveDate>,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision_comprobantes: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub firmante: Option<Firmante>,
    pub comprobantes: Vec<VoidedDocumentsItem>,
    /// Computed during enrichment: "RA-YYYYMMDD-{numero}"
    pub documento_id: Option<String>,
}

/// Detalle de un comprobante dado de baja
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct VoidedDocumentsItem {
    /// Catalog1: auto-filled from serie prefix (F->"01", B->"03")
    pub tipo_comprobante: Option<String>,
    pub serie: String,
    pub numero: u32,
    pub descripcion_sustento: String,
}
