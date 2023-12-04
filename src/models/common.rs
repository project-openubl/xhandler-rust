use std::fmt::Debug;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Proveedor {
    pub ruc: &'static str,
    pub razon_social: &'static str,
    pub nombre_comercial: Option<&'static str>,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Firmante {
    pub ruc: &'static str,
    pub razon_social: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Cliente {
    /// Catalog6
    pub tipo_documento_identidad: &'static str,
    pub numero_documento_identidad: &'static str,
    pub nombre: &'static str,
    pub direccion: Option<Direccion>,
    pub contacto: Option<Contacto>,
}

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

#[derive(Clone, Debug, Serialize)]
pub struct Contacto {
    pub telefono: &'static str,
    pub email: &'static str,
}
