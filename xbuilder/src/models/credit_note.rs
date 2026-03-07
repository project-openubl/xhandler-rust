use std::collections::HashMap;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::common::{
    Cliente, Detalle, DocumentoRelacionado, Firmante, Guia, Proveedor, TotalImporteNote,
    TotalImpuestos,
};

/// Nota de credito
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct CreditNote {
    pub leyendas: HashMap<String, String>,

    pub serie_numero: String,
    pub moneda: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    pub icb_tasa: Option<Decimal>,
    pub igv_tasa: Option<Decimal>,
    pub ivap_tasa: Option<Decimal>,

    /// Catalog9
    pub tipo_nota: Option<String>,
    pub comprobante_afectado_serie_numero: String,
    /// Catalog1
    pub comprobante_afectado_tipo: Option<String>,
    pub sustento_descripcion: String,

    pub detalles: Vec<Detalle>,

    pub total_importe: Option<TotalImporteNote>,
    pub total_impuestos: Option<TotalImpuestos>,

    pub guias: Vec<Guia>,
    pub documentos_relacionados: Vec<DocumentoRelacionado>,

    pub orden_de_compra: Option<String>,
}
