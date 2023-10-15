use std::fmt::Debug;

use chrono::NaiveDate;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Detraccion {
    /// Catalog59
    pub medio_de_pago: &'static str,
    pub cuenta_bancaria: &'static str,

    /// Catalog54
    pub tipo_bien_detraido: &'static str,
    pub porcentaje: f64,
    pub monto: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub enum TipoFormaDePago {
    Credito,
    Contado,
}

#[derive(Clone, Debug, Serialize)]
pub struct FormaDePago {
    pub tipo: TipoFormaDePago,
    pub cuotas: Vec<CuotaDePago>,
    pub total: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CuotaDePago {
    pub importe: f64,
    pub fecha_pago: NaiveDate,
}

#[derive(Clone, Debug, Serialize)]
pub struct Percepcion {
    /// Catalog53
    pub tipo: &'static str,

    pub porcentaje: f64,
    pub monto: Option<f64>,
    pub monto_base: Option<f64>,
    pub monto_total: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Anticipo {
    /// Catalog53. Valores válidos: "04", "05", "06"
    pub tipo: Option<&'static str>,

    /// Serie y número de comprobante del detalle, por ejemplo "F123-4"
    pub comprobante_serie_numero: &'static str,

    /// Catalog12.
    pub comprobante_tipo: Option<&'static str>,

    pub monto: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Descuento {
    /// Catalog53.
    pub tipo: Option<&'static str>,
    pub monto: f64,
    pub monto_base: Option<f64>,
    pub factor: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Guia {
    /// Catalog1. Valores válidos: "09", "31"
    pub tipo_documento: f64,
    pub serie_numero: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct DocumentoRelacionado {
    /// Catalog12.
    pub tipo_documento: f64,
    pub serie_numero: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Detalle {
    pub descripcion: &'static str,
    pub cantidad: f64,
    pub unidad_medida: Option<&'static str>,

    /// Precio + bool. True si el precio incluye impuestos, false si no incluye impuestos
    pub precio: Option<f64>,
    pub precio_con_impuestos: Option<f64>,
    pub precio_referencia: Option<f64>,
    /// Catalog16
    pub precio_referencia_tipo: Option<&'static str>,

    pub igv_tasa: Option<f64>,
    pub icb_tasa: Option<f64>,
    pub isc_tasa: Option<f64>,

    /// Catalog7
    pub igv_tipo: Option<&'static str>,
    /// Catalog8
    pub isc_tipo: Option<&'static str>,

    pub icb_aplica: bool,
    pub icb: Option<f64>,

    pub igv: Option<f64>,
    pub igv_base_imponible: Option<f64>,

    pub isc: Option<f64>,
    pub isc_base_imponible: Option<f64>,

    pub total_impuestos: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TotalImporte {
    pub anticipos: f64,
    pub descuentos: f64,
    pub importe: f64,
    pub importe_sin_impuestos: f64,
    pub importe_con_impuestos: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct TotalImpuestos {
    pub total: f64,
    pub ivap_importe: f64,
    pub ivap_base_imponible: f64,
    pub exportacion_importe: f64,
    pub exportacion_base_imponible: f64,
    pub gravado_importe: f64,
    pub gravado_base_imponible: f64,
    pub inafecto_importe: f64,
    pub inafecto_base_imponible: f64,
    pub exonerado_importe: f64,
    pub exonerado_base_imponible: f64,
    pub gratuito_importe: f64,
    pub gratuito_base_imponible: f64,
    pub icb_importe: f64,
    pub isc_importe: f64,
    pub isc_base_imponible: f64,
}
