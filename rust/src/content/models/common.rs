use derive_builder::Builder;

#[derive(Builder, Clone)]
pub struct Proveedor {
    pub ruc: String,
    pub razon_social: String,

    #[builder(default)]
    pub nombre_comercial: Option<String>,

    #[builder(default)]
    pub direccion: Option<Direccion>,

    #[builder(default)]
    pub contacto: Option<Contacto>,
}

#[derive(Builder, Clone)]
pub struct Firmante {
    pub ruc: String,
    pub razon_social: String,
}

#[derive(Builder, Clone)]
pub struct Direccion {
    pub ubigeo: String,
    pub codigo_local: String,
    pub urbanizacion: String,
    pub departamento: String,
    pub provincia: String,
    pub distrito: String,
    pub direccion: String,
    pub codigo_pais: String,
}

#[derive(Builder, Clone)]
pub struct Contacto {
    pub telefono: String,
    pub email: String,
}
