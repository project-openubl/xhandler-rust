use std::fmt::Debug;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Quien provee el documento electronico. Quien genera o crea el documento.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Proveedor {
    pub ruc: String,
    pub razon_social: String,
    pub nombre_comercial: Option<String>,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

/// Quien firma electronicamente el XML
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Firmante {
    pub ruc: String,
    pub razon_social: String,
}

/// Quien es el cliente de la transaccion
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Cliente {
    /// Catalog6
    pub tipo_documento_identidad: String,
    pub numero_documento_identidad: String,
    pub nombre: String,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

/// Direccion
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Direccion {
    pub ubigeo: Option<String>,
    pub codigo_local: Option<String>,
    pub urbanizacion: Option<String>,
    pub departamento: Option<String>,
    pub provincia: Option<String>,
    pub distrito: Option<String>,
    pub direccion: Option<String>,
    pub codigo_pais: Option<String>,
}

/// Contacto
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contacto {
    pub telefono: String,
    pub email: String,
}

/// Detraccion
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Detraccion {
    /// Catalog59
    pub medio_de_pago: String,
    pub cuenta_bancaria: String,

    /// Catalog54
    pub tipo_bien_detraido: String,
    pub porcentaje: Decimal,
    pub monto: Option<Decimal>,
}

/// Tipo de forma de pago
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TipoFormaDePago {
    Credito,
    Contado,
}

/// Forma de pago
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormaDePago {
    pub tipo: Option<TipoFormaDePago>,
    pub cuotas: Vec<CuotaDePago>,
    pub total: Option<Decimal>,
}

/// Cuota de pago
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CuotaDePago {
    pub importe: Decimal,
    pub fecha_pago: NaiveDate,
}

/// Percepcion asociada a un documento
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Percepcion {
    /// Catalog53
    pub tipo: String,

    pub porcentaje: Option<Decimal>,
    pub monto: Option<Decimal>,
    pub monto_base: Option<Decimal>,
    pub monto_total: Option<Decimal>,
}

/// Anticipo asociado a un documento
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anticipo {
    /// Catalog53. Valores válidos: "04", "05", "06"
    pub tipo: Option<String>,

    /// Serie y número de comprobante del detalle, por ejemplo "F123-4"
    pub comprobante_serie_numero: String,

    /// Catalog12.
    pub comprobante_tipo: Option<String>,

    pub monto: Decimal,
}

/// Descuento asociado a un documento o item
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Descuento {
    /// Catalog53.
    pub tipo: Option<String>,
    pub monto: Decimal,
    pub monto_base: Option<Decimal>,
    pub factor: Option<Decimal>,
}

/// Guia asociada a un documento
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guia {
    /// Catalog1. Valores válidos: "09", "31"
    pub tipo_documento: String,
    pub serie_numero: String,
}

/// Documento relacionado
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentoRelacionado {
    /// Catalog12.
    pub tipo_documento: String,
    pub serie_numero: String,
}

/// Código estándar GS1/GTIN para identificación de producto
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodigoGS1 {
    /// Código GS1/GTIN del producto
    pub codigo: String,
    /// Tipo de estructura GTIN: e.g. "GTIN-8", "GTIN-13", "GTIN-14"
    pub tipo: String,
}

/// Atributo adicional de un detalle
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Atributo {
    pub nombre: String,
    pub codigo: String,
    pub valor: Option<String>,
    pub fecha_inicio: Option<NaiveDate>,
    pub fecha_fin: Option<NaiveDate>,
    pub duracion: Option<u32>,
}

/// Detalle de las ventas en Boleta/Factura/Nota Credito/Nota Debito
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Detalle {
    pub descripcion: String,
    pub cantidad: Decimal,
    pub unidad_medida: Option<String>,

    /// Código del producto del vendedor (SellersItemIdentification)
    pub codigo: Option<String>,
    /// Código de producto SUNAT - Catalog25 (CommodityClassification)
    pub codigo_sunat: Option<String>,
    /// Código estándar GS1/GTIN (StandardItemIdentification).
    /// Incluye el código y el tipo de estructura GTIN (e.g. "GTIN-8", "GTIN-13", "GTIN-14").
    pub codigo_gs1: Option<CodigoGS1>,

    /// Precio + bool. True si el precio incluye impuestos, false si no incluye impuestos
    pub precio: Option<Decimal>,
    pub precio_con_impuestos: Option<Decimal>,
    pub precio_referencia: Option<Decimal>,
    /// Catalog16
    pub precio_referencia_tipo: Option<String>,

    pub igv_tasa: Option<Decimal>,
    pub icb_tasa: Option<Decimal>,
    pub isc_tasa: Option<Decimal>,

    /// Catalog7
    pub igv_tipo: Option<String>,
    /// Catalog8
    pub isc_tipo: Option<String>,

    pub icb_aplica: bool,
    pub icb: Option<Decimal>,

    pub igv: Option<Decimal>,
    pub igv_base_imponible: Option<Decimal>,

    pub isc: Option<Decimal>,
    pub isc_base_imponible: Option<Decimal>,

    pub total_impuestos: Option<Decimal>,

    pub atributos: Vec<Atributo>,
}

/// Total Importe Invoice
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TotalImporteInvoice {
    pub anticipos: Decimal,
    pub descuentos: Decimal,
    pub importe: Decimal,
    pub importe_sin_impuestos: Decimal,
    pub importe_con_impuestos: Decimal,
}

// Total importe Note
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TotalImporteNote {
    pub importe: Decimal,
    pub importe_sin_impuestos: Decimal,
}

/// Total impuestos
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TotalImpuestos {
    pub total: Decimal,
    pub ivap_importe: Decimal,
    pub ivap_base_imponible: Decimal,
    pub exportacion_importe: Decimal,
    pub exportacion_base_imponible: Decimal,
    pub gravado_importe: Decimal,
    pub gravado_base_imponible: Decimal,
    pub inafecto_importe: Decimal,
    pub inafecto_base_imponible: Decimal,
    pub exonerado_importe: Decimal,
    pub exonerado_base_imponible: Decimal,
    pub gratuito_importe: Decimal,
    pub gratuito_base_imponible: Decimal,
    pub icb_importe: Decimal,
    pub isc_importe: Decimal,
    pub isc_base_imponible: Decimal,
}
