use std::fmt::Debug;

use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub enum PorcentajeMonto {
    PorcentajeMonto(f32, f64),
    Porcentaje(f32),
    Monto(f64),
}

#[derive(Clone, Debug)]
pub struct Detraccion {
    /// Catalog59
    pub medio_de_pago: &'static str,
    pub cuenta_bancaria: &'static str,
    /// Catalog54
    pub tipo_bien_detraido: &'static str,
    pub porcentaje_monto: PorcentajeMonto,
}

#[derive(Clone, Debug)]
pub enum TipoFormaDePago {
    Credito,
    Contado,
}

#[derive(Clone, Debug)]
pub struct FormaDePago {
    pub tipo: TipoFormaDePago,
    pub cuotas: Vec<CuotaDePago>,
    pub total: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct CuotaDePago {
    pub importe: f64,
    pub fecha_pago: NaiveDate,
}

#[derive(Clone, Debug)]
pub struct Percepcion {
    /// Catalog53
    pub tipo: &'static str,
}

#[derive(Clone, Debug)]
pub struct Anticipo {
    /// Catalog53. Valores válidos: "04", "05", "06"
    pub tipo: Option<&'static str>,

    /// Serie y número de comprobante del anticipo, por ejemplo "F123-4"
    pub comprobante_serie_numero: &'static str,

    /// Catalog12.
    pub comprobante_tipo: Option<&'static str>,

    pub monto: f64,
}

#[derive(Clone, Debug)]
pub struct Descuento {
    /// Catalog53.
    pub tipo: Option<&'static str>,
    pub monto: f64,
    pub monto_base: Option<f64>,
    pub factor: Option<f64>,
}
