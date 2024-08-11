use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{
    Anticipo, Cliente, Descuento, Detalle, Detraccion, Direccion, DocumentoRelacionado, Firmante,
    FormaDePago, Guia, Percepcion, Proveedor, TotalImporteInvoice, TotalImpuestos,
};

/// Boleta o Factura
#[derive(Debug, Serialize, Default)]
pub struct Invoice {
    pub leyendas: HashMap<&'static str, &'static str>,

    pub serie_numero: &'static str,
    pub moneda: Option<&'static str>,
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
    pub tipo_comprobante: Option<&'static str>,

    /// Catalog51
    pub tipo_operacion: Option<&'static str>,

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

    pub observaciones: Option<&'static str>,
    pub orden_de_compra: Option<&'static str>,
}
