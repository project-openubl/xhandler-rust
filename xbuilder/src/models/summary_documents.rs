use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{Firmante, Proveedor};

/// Resumen diario de boletas
#[derive(Debug, Serialize, Default)]
pub struct SummaryDocuments {
    pub numero: u32,
    pub fecha_emision: Option<NaiveDate>,
    pub fecha_emision_comprobantes: Option<NaiveDate>,
    pub moneda: Option<&'static str>,
    pub proveedor: Proveedor,
    pub firmante: Option<Firmante>,
    pub comprobantes: Vec<SummaryDocumentsItem>,
    /// Computed during enrichment: "RC-YYYYMMDD-{numero}"
    pub documento_id: Option<String>,
}

/// Detalle de un comprobante en el resumen
#[derive(Clone, Debug, Serialize, Default)]
pub struct SummaryDocumentsItem {
    /// Catalog19: "1"=Adicionar, "2"=Modificar, "3"=Anulado
    pub tipo_operacion: &'static str,
    pub comprobante: SummaryDocumentsItemComprobante,
}

/// Comprobante dentro de SummaryDocumentsItem
#[derive(Clone, Debug, Serialize, Default)]
pub struct SummaryDocumentsItemComprobante {
    /// Catalog1: "03"=Boleta, "07"=NC, "08"=ND
    pub tipo_comprobante: &'static str,
    pub serie_numero: &'static str,
    pub moneda: Option<&'static str>,
    pub cliente: SummaryDocumentsCliente,
    pub comprobante_afectado: Option<SummaryDocumentsComprobanteAfectado>,
    pub valor_venta: SummaryDocumentsValorVenta,
    pub impuestos: SummaryDocumentsImpuestos,
}

/// Cliente simplificado para SummaryDocuments
#[derive(Clone, Debug, Serialize, Default)]
pub struct SummaryDocumentsCliente {
    pub numero_documento_identidad: &'static str,
    pub tipo_documento_identidad: &'static str,
}

/// Comprobante afectado (para NC/ND en el resumen)
#[derive(Clone, Debug, Serialize)]
pub struct SummaryDocumentsComprobanteAfectado {
    pub tipo_comprobante: &'static str,
    pub serie_numero: &'static str,
}

/// Valor de venta de un comprobante en el resumen
#[derive(Clone, Debug, Serialize, Default)]
pub struct SummaryDocumentsValorVenta {
    pub importe_total: Decimal,
    pub gravado: Option<Decimal>,
    pub exonerado: Option<Decimal>,
    pub inafecto: Option<Decimal>,
    pub gratuito: Option<Decimal>,
    pub otros_cargos: Option<Decimal>,
}

/// Impuestos de un comprobante en el resumen
#[derive(Clone, Debug, Serialize, Default)]
pub struct SummaryDocumentsImpuestos {
    pub igv: Option<Decimal>,
    /// IGV rate as percentage (e.g., 18 for 18%). Required by SUNAT when igv is present.
    pub igv_tasa: Option<Decimal>,
    pub icb: Option<Decimal>,
}
