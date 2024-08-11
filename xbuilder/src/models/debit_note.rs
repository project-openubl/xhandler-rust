use std::collections::HashMap;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{
    Cliente, Detalle, DocumentoRelacionado, Firmante, Guia, Proveedor, TotalImporteNote,
    TotalImpuestos,
};

/// Nota de debito
#[derive(Debug, Serialize, Default)]
pub struct DebitNote {
    pub leyendas: HashMap<&'static str, &'static str>,

    pub serie_numero: &'static str,
    pub moneda: Option<&'static str>,
    pub fecha_emision: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    pub icb_tasa: Option<Decimal>,
    pub igv_tasa: Option<Decimal>,
    pub ivap_tasa: Option<Decimal>,

    // Catalog10
    pub tipo_nota: Option<&'static str>,
    pub comprobante_afectado_serie_numero: &'static str,
    // Catalog1
    pub comprobante_afectado_tipo: Option<&'static str>,
    pub sustento_descripcion: &'static str,

    pub detalles: Vec<Detalle>,

    pub total_importe: Option<TotalImporteNote>,
    pub total_impuestos: Option<TotalImpuestos>,

    pub guias: Vec<Guia>,
    pub documentos_relacionados: Vec<DocumentoRelacionado>,

    pub orden_de_compra: Option<&'static str>,
}
