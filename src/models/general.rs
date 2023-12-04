use std::fmt::Debug;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

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

#[derive(Clone, Debug, Serialize)]
pub enum TipoFormaDePago {
    Credito,
    Contado,
}

#[derive(Clone, Debug, Serialize)]
pub struct FormaDePago {
    pub tipo: Option<TipoFormaDePago>,
    pub cuotas: Vec<CuotaDePago>,
    pub total: Option<Decimal>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CuotaDePago {
    pub importe: Decimal,
    pub fecha_pago: NaiveDate,
}

#[derive(Clone, Debug, Serialize)]
pub struct Percepcion {
    /// Catalog53
    pub tipo: &'static str,

    pub porcentaje: Option<Decimal>,
    pub monto: Option<Decimal>,
    pub monto_base: Option<Decimal>,
    pub monto_total: Option<Decimal>,
}

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

#[derive(Clone, Debug, Serialize)]
pub struct Descuento {
    /// Catalog53.
    pub tipo: Option<&'static str>,
    pub monto: Decimal,
    pub monto_base: Option<Decimal>,
    pub factor: Option<Decimal>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Guia {
    /// Catalog1. Valores válidos: "09", "31"
    pub tipo_documento: Decimal,
    pub serie_numero: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct DocumentoRelacionado {
    /// Catalog12.
    pub tipo_documento: &'static str,
    pub serie_numero: &'static str,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub struct TotalImporte {
    pub anticipos: Decimal,
    pub descuentos: Decimal,
    pub importe: Decimal,
    pub importe_sin_impuestos: Decimal,
    pub importe_con_impuestos: Decimal,
}

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
