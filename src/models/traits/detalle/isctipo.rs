use crate::models::general::Detalle;

pub trait DetalleISCTipoGetter {
    fn get_isctipo(&self) -> &Option<&'static str>;
}

pub trait DetalleISCTipoSetter {
    fn set_isctipo(&mut self, val: &'static str);
}

impl DetalleISCTipoGetter for Detalle {
    fn get_isctipo(&self) -> &Option<&'static str> {
        &self.isc_tipo
    }
}

impl DetalleISCTipoSetter for Detalle {
    fn set_isctipo(&mut self, val: &'static str) {
        self.isc_tipo = Some(val);
    }
}
