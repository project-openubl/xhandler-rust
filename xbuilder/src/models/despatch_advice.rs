use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::models::common::{Firmante, Proveedor};

/// Guia de Remision
#[derive(Debug, Serialize, Default)]
pub struct DespatchAdvice {
    pub serie_numero: &'static str,
    pub fecha_emision: Option<NaiveDate>,
    pub hora_emision: Option<NaiveTime>,
    /// Catalog1: auto-filled from serie prefix (T->"09", V->"31")
    pub tipo_comprobante: Option<&'static str>,
    pub observaciones: Option<&'static str>,
    pub documento_baja: Option<DespatchAdviceDocumentoBaja>,
    pub documento_relacionado: Option<DespatchAdviceDocumentoRelacionado>,
    pub firmante: Option<Firmante>,
    pub remitente: Remitente,
    pub destinatario: Destinatario,
    pub proveedor: Option<Proveedor>,
    pub envio: Envio,
    pub detalles: Vec<DespatchAdviceItem>,
}

/// Remitente (emisor de la guia)
#[derive(Clone, Debug, Serialize, Default)]
pub struct Remitente {
    pub ruc: &'static str,
    pub razon_social: &'static str,
}

/// Destinatario (receptor de la mercancia)
#[derive(Clone, Debug, Serialize, Default)]
pub struct Destinatario {
    pub tipo_documento_identidad: &'static str,
    pub numero_documento_identidad: &'static str,
    pub nombre: &'static str,
}

/// Informacion del envio
#[derive(Clone, Debug, Serialize, Default)]
pub struct Envio {
    /// Catalog20: motivo de traslado
    pub tipo_traslado: &'static str,
    pub motivo_traslado: Option<&'static str>,
    pub peso_total: Decimal,
    /// Computed: peso_total formatted to 3 decimal places
    pub peso_total_formatted: Option<String>,
    pub peso_total_unidad_medida: &'static str,
    pub numero_de_bultos: Option<u32>,
    /// Catalog18: modalidad de traslado (01=publico, 02=privado)
    pub tipo_modalidad_traslado: &'static str,
    pub fecha_traslado: NaiveDate,
    pub transbordo_programado: bool,
    pub transportista: Option<Transportista>,
    pub numero_de_contenedor: Option<&'static str>,
    pub codigo_de_puerto: Option<&'static str>,
    pub partida: Partida,
    pub destino: Destino,
}

/// Transportista
#[derive(Clone, Debug, Serialize, Default)]
pub struct Transportista {
    pub tipo_documento_identidad: &'static str,
    pub numero_documento_identidad: &'static str,
    pub nombre: &'static str,
    pub placa_del_vehiculo: &'static str,
    pub chofer_tipo_documento_identidad: &'static str,
    pub chofer_numero_documento_identidad: &'static str,
}

/// Punto de partida
#[derive(Clone, Debug, Serialize, Default)]
pub struct Partida {
    pub ubigeo: &'static str,
    pub direccion: &'static str,
}

/// Punto de destino
#[derive(Clone, Debug, Serialize, Default)]
pub struct Destino {
    pub ubigeo: &'static str,
    pub direccion: &'static str,
}

/// Item de la guia de remision
#[derive(Clone, Debug, Serialize, Default)]
pub struct DespatchAdviceItem {
    pub unidad_medida: &'static str,
    pub cantidad: Decimal,
    pub descripcion: Option<&'static str>,
    pub codigo: &'static str,
    pub codigo_sunat: Option<&'static str>,
}

/// Documento de baja asociado
#[derive(Clone, Debug, Serialize)]
pub struct DespatchAdviceDocumentoBaja {
    pub tipo_documento: &'static str,
    pub serie_numero: &'static str,
}

/// Documento relacionado
#[derive(Clone, Debug, Serialize)]
pub struct DespatchAdviceDocumentoRelacionado {
    pub tipo_documento: &'static str,
    pub serie_numero: &'static str,
}
