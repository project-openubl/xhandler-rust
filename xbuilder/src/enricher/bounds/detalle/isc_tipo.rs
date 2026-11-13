use crate::models::common::Detalle;

pub trait DetalleIscTipoGetter {
    fn isc_tipo(&self) -> &Option<String>;
}

pub trait DetalleIscTipoSetter {
    fn set_isc_tipo(&mut self, val: &str);
}

impl DetalleIscTipoGetter for Detalle {
    fn isc_tipo(&self) -> &Option<String> {
        &self.isc_tipo
    }
}

impl DetalleIscTipoSetter for Detalle {
    fn set_isc_tipo(&mut self, val: &str) {
        self.isc_tipo = Some(val.to_string());
    }
}
