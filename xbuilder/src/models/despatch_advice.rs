use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::common::{Firmante, Proveedor};

/// Guia de Remision (GRE)
#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DespatchAdvice {
    pub serie_numero: String,
    /// UBL CustomizationID, "2.0" when missing
    pub version: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_emision: Option<NaiveDate>,
    pub hora_emision: Option<NaiveTime>,
    /// Catalog1: auto-filled from serie prefix (T->"09", V->"31")
    pub tipo_comprobante: Option<String>,
    pub observaciones: Option<String>,
    pub documento_baja: Option<DespatchAdviceDocumentoBaja>,
    pub documento_relacionado: Option<DespatchAdviceDocumentoRelacionado>,
    pub documentos_adicionales: Vec<DocumentoAdicional>,
    pub firmante: Option<Firmante>,
    pub remitente: Remitente,
    pub destinatario: Destinatario,
    pub comprador: Option<Comprador>,
    /// Remitente original when a third party issues the guia (e.g. the carrier in a "31")
    pub tercero: Option<Tercero>,
    /// Rendered as SellerSupplierParty when there is no tercero
    pub proveedor: Option<Proveedor>,
    pub envio: Envio,
    pub detalles: Vec<DespatchAdviceItem>,
}

/// Remitente (emisor de la guia)
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Remitente {
    pub ruc: String,
    pub razon_social: String,
    pub numero_registro_mtc: Option<String>,
}

/// Persona identificada por su documento y nombre: el destinatario de la
/// mercancia, y tambien el comprador y el tercero
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Destinatario {
    /// Catalog6
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombre: String,
}

/// Comprador, cuando es distinto del destinatario
pub type Comprador = Destinatario;

/// Tercero que solicita el traslado
pub type Tercero = Destinatario;

/// Documento adicional relacionado al transporte
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DocumentoAdicional {
    /// Catalog61
    pub tipo_documento: String,
    pub tipo_documento_descripcion: Option<String>,
    pub numero: String,
    pub ruc_emisor: Option<String>,
}

/// Informacion del envio
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Envio {
    /// Catalog20: motivo de traslado
    pub tipo_traslado: String,
    pub motivo_traslado: Option<String>,
    pub sustento_peso: Option<String>,
    pub peso_total: Decimal,
    pub peso_total_unidad_medida: String,
    /// Peso neto de los items, en KGM
    pub peso_items: Option<Decimal>,
    pub numero_bultos: Option<u32>,
    /// Indicadores especiales (SpecialInstructions)
    pub indicadores: Vec<String>,
    /// Catalog18: modalidad de traslado (01=publico, 02=privado)
    pub tipo_modalidad_traslado: String,
    /// Fecha de inicio del traslado; obligatoria
    #[serde(default, deserialize_with = "crate::serde_date::option::deserialize")]
    pub fecha_traslado: Option<NaiveDate>,
    pub transportista: Option<Transportista>,
    pub choferes: Vec<Chofer>,
    pub vehiculo: Option<Vehiculo>,
    pub contenedores: Vec<String>,
    /// Catalog63
    pub puerto: Option<Puerto>,
    /// Catalog64, used when there is no puerto
    pub aeropuerto: Option<Puerto>,
    pub partida: Option<Partida>,
    pub destino: Option<Destino>,
}

/// Transportista (transporte publico)
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Transportista {
    /// Catalog6
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombre: String,
    pub numero_registro_mtc: Option<String>,
}

/// Conductor del vehiculo
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Chofer {
    /// Free text rendered as JobTitle, e.g. "CONDUCTOR_PRINCIPAL", "COPILOTO"
    pub tipo: Option<String>,
    /// Catalog6
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombres: Option<String>,
    pub apellidos: Option<String>,
    pub licencia: Option<String>,
}

/// Vehiculo principal, o secundario dentro de `secundarios`
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Vehiculo {
    pub placa: String,
    /// Tarjeta unica de circulacion (TUC)
    pub numero_circulacion: Option<String>,
    pub numero_autorizacion: Option<String>,
    /// Entidad que emite la autorizacion
    pub codigo_emisor: Option<String>,
    pub secundarios: Vec<Vehiculo>,
}

/// Puerto o aeropuerto
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Puerto {
    pub codigo: String,
    pub nombre: String,
}

/// Punto de partida
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Partida {
    pub ubigeo: String,
    pub direccion: String,
    /// Codigo de establecimiento anexo, del `ruc` indicado
    pub codigo_local: Option<String>,
    pub ruc: Option<String>,
}

/// Punto de llegada
pub type Destino = Partida;

/// Item de la guia de remision
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DespatchAdviceItem {
    pub unidad_medida: String,
    pub cantidad: Decimal,
    pub descripcion: Option<String>,
    pub codigo: String,
    pub codigo_sunat: Option<String>,
    pub atributos: Vec<DespatchAdviceAtributo>,
}

/// Propiedad adicional de un item (AdditionalItemProperty)
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DespatchAdviceAtributo {
    pub codigo: String,
    pub nombre: String,
    pub valor: Option<String>,
}

/// Documento de baja asociado
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DespatchAdviceDocumentoBaja {
    /// Catalog1
    pub tipo_comprobante: String,
    pub serie_numero: String,
}

/// Documento relacionado
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DespatchAdviceDocumentoRelacionado {
    pub tipo_documento: String,
    pub serie_numero: String,
}
