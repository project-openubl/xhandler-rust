use std::fmt::Debug;

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
