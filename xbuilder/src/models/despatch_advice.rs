use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::common::{Firmante, Proveedor};

/// Guia de Remision
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct DespatchAdvice {
    pub serie_numero: String,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision: Option<NaiveDate>,
    pub hora_emision: Option<NaiveTime>,
    /// Catalog1: auto-filled from serie prefix (T->"09", V->"31")
    pub tipo_comprobante: Option<String>,
    pub observaciones: Option<String>,
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
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Remitente {
    pub ruc: String,
    pub razon_social: String,
}

/// Destinatario (receptor de la mercancia)
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Destinatario {
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombre: String,
}

/// Informacion del envio
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Envio {
    /// Catalog20: motivo de traslado
    pub tipo_traslado: String,
    pub motivo_traslado: Option<String>,
    pub peso_total: Decimal,
    /// Computed: peso_total formatted to 3 decimal places
    pub peso_total_formatted: Option<String>,
    pub peso_total_unidad_medida: String,
    pub numero_de_bultos: Option<u32>,
    /// Catalog18: modalidad de traslado (01=publico, 02=privado)
    pub tipo_modalidad_traslado: String,
    #[serde(deserialize_with = "crate::serde_date::deserialize")]
    pub fecha_traslado: NaiveDate,
    pub transbordo_programado: bool,
    pub transportista: Option<Transportista>,
    pub numero_de_contenedor: Option<String>,
    pub codigo_de_puerto: Option<String>,
    pub partida: Partida,
    pub destino: Destino,
}

/// Transportista
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Transportista {
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombre: String,
    pub placa_del_vehiculo: String,
    pub chofer_tipo_documento_identidad: String,
    pub chofer_numero_documento_identidad: String,
}

/// Punto de partida
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Partida {
    pub ubigeo: String,
    pub direccion: String,
}

/// Punto de destino
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Destino {
    pub ubigeo: String,
    pub direccion: String,
}

/// Item de la guia de remision
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct DespatchAdviceItem {
    pub unidad_medida: String,
    pub cantidad: Decimal,
    pub descripcion: Option<String>,
    pub codigo: String,
    pub codigo_sunat: Option<String>,
}

/// Documento de baja asociado
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DespatchAdviceDocumentoBaja {
    pub tipo_documento: String,
    pub serie_numero: String,
}

/// Documento relacionado
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DespatchAdviceDocumentoRelacionado {
    pub tipo_documento: String,
    pub serie_numero: String,
}
