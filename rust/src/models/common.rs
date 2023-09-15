use std::fmt::Debug;

use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Proveedor {
    pub ruc: &'static str,
    pub razon_social: &'static str,
    pub nombre_comercial: Option<&'static str>,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

#[derive(Clone, Debug)]
pub struct Firmante {
    pub ruc: &'static str,
    pub razon_social: &'static str,
}

#[derive(Clone, Debug)]
pub struct Cliente {
    /// Catalog6
    pub tipo_documento_identidad: &'static str,
    pub numero_documento_identidad: &'static str,
    pub nombre: &'static str,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

#[derive(Clone, Debug)]
pub struct Direccion {
    pub ubigeo: Option<&'static str>,
    pub codigo_local: &'static str,
    pub urbanizacion: Option<&'static str>,
    pub departamento: Option<&'static str>,
    pub provincia: Option<&'static str>,
    pub distrito: Option<&'static str>,
    pub direccion: Option<&'static str>,
    pub codigo_pais: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Contacto {
    pub telefono: &'static str,
    pub email: &'static str,
}

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
