use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::common::{Firmante, Proveedor};

/// Resumen diario de boletas
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocuments {
    pub numero: u32,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision: Option<NaiveDate>,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision_comprobantes: Option<NaiveDate>,
    pub moneda: Option<String>,
    pub proveedor: Proveedor,
    pub firmante: Option<Firmante>,
    pub comprobantes: Vec<SummaryDocumentsItem>,
    /// Computed during enrichment: "RC-YYYYMMDD-{numero}"
    pub documento_id: Option<String>,
}

/// Detalle de un comprobante en el resumen
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocumentsItem {
    /// Catalog19: "1"=Adicionar, "2"=Modificar, "3"=Anulado
    pub tipo_operacion: String,
    pub comprobante: SummaryDocumentsItemComprobante,
}

/// Comprobante dentro de SummaryDocumentsItem
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocumentsItemComprobante {
    /// Catalog1: "03"=Boleta, "07"=NC, "08"=ND
    pub tipo_comprobante: String,
    pub serie_numero: String,
    pub moneda: Option<String>,
    pub cliente: SummaryDocumentsCliente,
    pub comprobante_afectado: Option<SummaryDocumentsComprobanteAfectado>,
    pub valor_venta: SummaryDocumentsValorVenta,
    pub impuestos: SummaryDocumentsImpuestos,
}

/// Cliente simplificado para SummaryDocuments
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocumentsCliente {
    pub numero_documento_identidad: String,
    pub tipo_documento_identidad: String,
}

/// Comprobante afectado (para NC/ND en el resumen)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SummaryDocumentsComprobanteAfectado {
    pub tipo_comprobante: String,
    pub serie_numero: String,
}

/// Valor de venta de un comprobante en el resumen
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocumentsValorVenta {
    pub importe_total: Decimal,
    pub gravado: Option<Decimal>,
    pub exonerado: Option<Decimal>,
    pub inafecto: Option<Decimal>,
    pub gratuito: Option<Decimal>,
    pub otros_cargos: Option<Decimal>,
}

/// Impuestos de un comprobante en el resumen
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SummaryDocumentsImpuestos {
    pub igv: Option<Decimal>,
    /// IGV rate as percentage (e.g., 18 for 18%). Required by SUNAT when igv is present.
    pub igv_tasa: Option<Decimal>,
    pub icb: Option<Decimal>,
}
