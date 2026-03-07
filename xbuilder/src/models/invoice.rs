use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::common::{
    Anticipo, Cliente, Descuento, Detalle, Detraccion, Direccion, DocumentoRelacionado, Firmante,
    FormaDePago, Guia, Percepcion, Proveedor, TotalImporteInvoice, TotalImpuestos,
};

/// Boleta o Factura
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Invoice {
    pub leyendas: HashMap<String, String>,

    pub serie_numero: String,
    pub moneda: Option<String>,
    pub fecha_emision: Option<NaiveDate>,
    pub hora_emision: Option<NaiveTime>,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub proveedor: Proveedor,
    pub cliente: Cliente,
    pub firmante: Option<Firmante>,
    pub icb_tasa: Option<Decimal>,
    pub igv_tasa: Option<Decimal>,
    pub ivap_tasa: Option<Decimal>,

    /// Catalog1
    pub tipo_comprobante: Option<String>,

    /// Catalog51
    pub tipo_operacion: Option<String>,

    pub detraccion: Option<Detraccion>,
    pub percepcion: Option<Percepcion>,

    pub direccion_entrega: Option<Direccion>,
    pub forma_de_pago: Option<FormaDePago>,

    pub anticipos: Vec<Anticipo>,
    pub descuentos: Vec<Descuento>,

    pub detalles: Vec<Detalle>,

    pub total_importe: Option<TotalImporteInvoice>,
    pub total_impuestos: Option<TotalImpuestos>,

    pub guias: Vec<Guia>,
    pub documentos_relacionados: Vec<DocumentoRelacionado>,

    pub observaciones: Option<String>,
    pub orden_de_compra: Option<String>,
}
