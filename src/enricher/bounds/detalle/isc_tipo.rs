use crate::models::common::Detalle;

pub trait DetalleIscTipoGetter {
    fn get_isc_tipo(&self) -> &Option<&'static str>;
}

pub trait DetalleIscTipoSetter {
    fn set_isc_tipo(&mut self, val: &'static str);
}

impl DetalleIscTipoGetter for Detalle {
    fn get_isc_tipo(&self) -> &Option<&'static str> {
        &self.isc_tipo
    }
}

impl DetalleIscTipoSetter for Detalle {
    fn set_isc_tipo(&mut self, val: &'static str) {
        self.isc_tipo = Some(val);
    }
}
