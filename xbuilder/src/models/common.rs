use std::fmt::Debug;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

/// Quien provee el documento electronico. Quien genera o crea el documento.
#[derive(Clone, Debug, Serialize, Default)]
pub struct Proveedor {
    pub ruc: &'static str,
    pub razon_social: &'static str,
    pub nombre_comercial: Option<&'static str>,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

/// Quien firma electronicamente el XML
#[derive(Clone, Debug, Serialize)]
pub struct Firmante {
    pub ruc: &'static str,
    pub razon_social: &'static str,
}

/// Quien es el cliente de la transaccion
#[derive(Clone, Debug, Serialize, Default)]
pub struct Cliente {
    /// Catalog6
    pub tipo_documento_identidad: &'static str,
    pub numero_documento_identidad: &'static str,
    pub nombre: &'static str,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

/// Direccion
#[derive(Clone, Debug, Serialize)]
pub struct Direccion {
    pub ubigeo: Option<&'static str>,
    pub codigo_local: Option<&'static str>,
    pub urbanizacion: Option<&'static str>,
    pub departamento: Option<&'static str>,
    pub provincia: Option<&'static str>,
    pub distrito: Option<&'static str>,
    pub direccion: Option<&'static str>,
    pub codigo_pais: Option<&'static str>,
}

/// Contacto
#[derive(Clone, Debug, Serialize)]
pub struct Contacto {
    pub telefono: &'static str,
    pub email: &'static str,
}

/// Detraccion
#[derive(Clone, Debug, Serialize)]
pub struct Detraccion {
    /// Catalog59
    pub medio_de_pago: &'static str,
    pub cuenta_bancaria: &'static str,

    /// Catalog54
    pub tipo_bien_detraido: &'static str,
    pub porcentaje: Decimal,
    pub monto: Option<Decimal>,
}

/// Tipo de forma de pago
#[derive(Clone, Debug, Serialize)]
pub enum TipoFormaDePago {
    Credito,
    Contado,
}

/// Forma de pago
#[derive(Clone, Debug, Serialize)]
pub struct FormaDePago {
    pub tipo: Option<TipoFormaDePago>,
    pub cuotas: Vec<CuotaDePago>,
    pub total: Option<Decimal>,
}

/// Cuota de pago
#[derive(Clone, Debug, Serialize)]
pub struct CuotaDePago {
    pub importe: Decimal,
    pub fecha_pago: NaiveDate,
}

/// Percepcion asociada a un documento
#[derive(Clone, Debug, Serialize)]
pub struct Percepcion {
    /// Catalog53
    pub tipo: &'static str,

    pub porcentaje: Option<Decimal>,
    pub monto: Option<Decimal>,
    pub monto_base: Option<Decimal>,
    pub monto_total: Option<Decimal>,
}

/// Anticipo asociado a un documento
#[derive(Clone, Debug, Serialize)]
pub struct Anticipo {
    /// Catalog53. Valores válidos: "04", "05", "06"
    pub tipo: Option<&'static str>,

    /// Serie y número de comprobante del detalle, por ejemplo "F123-4"
    pub comprobante_serie_numero: &'static str,

    /// Catalog12.
    pub comprobante_tipo: Option<&'static str>,

    pub monto: Decimal,
}

/// Descuento asociado a un documento o item
#[derive(Clone, Debug, Serialize)]
pub struct Descuento {
    /// Catalog53.
    pub tipo: Option<&'static str>,
    pub monto: Decimal,
    pub monto_base: Option<Decimal>,
    pub factor: Option<Decimal>,
}

/// Guia asociada a un documento
#[derive(Clone, Debug, Serialize)]
pub struct Guia {
    /// Catalog1. Valores válidos: "09", "31"
    pub tipo_documento: &'static str,
    pub serie_numero: &'static str,
}

/// Documento relacionado
#[derive(Clone, Debug, Serialize)]
pub struct DocumentoRelacionado {
    /// Catalog12.
    pub tipo_documento: &'static str,
    pub serie_numero: &'static str,
}

/// Detalle de las ventas en Boleta/Factura/Nota Credito/Nota Debito
#[derive(Clone, Debug, Serialize, Default)]
pub struct Detalle {
    pub descripcion: &'static str,
    pub cantidad: Decimal,
    pub unidad_medida: Option<&'static str>,

    /// Precio + bool. True si el precio incluye impuestos, false si no incluye impuestos
    pub precio: Option<Decimal>,
    pub precio_con_impuestos: Option<Decimal>,
    pub precio_referencia: Option<Decimal>,
    /// Catalog16
    pub precio_referencia_tipo: Option<&'static str>,

    pub igv_tasa: Option<Decimal>,
    pub icb_tasa: Option<Decimal>,
    pub isc_tasa: Option<Decimal>,

    /// Catalog7
    pub igv_tipo: Option<&'static str>,
    /// Catalog8
    pub isc_tipo: Option<&'static str>,

    pub icb_aplica: bool,
    pub icb: Option<Decimal>,

    pub igv: Option<Decimal>,
    pub igv_base_imponible: Option<Decimal>,

    pub isc: Option<Decimal>,
    pub isc_base_imponible: Option<Decimal>,

    pub total_impuestos: Option<Decimal>,
}

/// Total Importe Invoice
#[derive(Clone, Debug, Serialize)]
pub struct TotalImporteInvoice {
    pub anticipos: Decimal,
    pub descuentos: Decimal,
    pub importe: Decimal,
    pub importe_sin_impuestos: Decimal,
    pub importe_con_impuestos: Decimal,
}

// Total importe Note
#[derive(Clone, Debug, Serialize)]
pub struct TotalImporteNote {
    pub importe: Decimal,
    pub importe_sin_impuestos: Decimal,
}

/// Total impuestos
#[derive(Clone, Debug, Serialize)]
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
