use crate::models::common::Detalle;

pub trait DetalleIgvTipoGetter {
    fn get_igv_tipo(&self) -> &Option<&'static str>;
}

pub trait DetalleIgvTipoSetter {
    fn set_igv_tipo(&mut self, val: &'static str);
}

impl DetalleIgvTipoGetter for Detalle {
    fn get_igv_tipo(&self) -> &Option<&'static str> {
        &self.igv_tipo
    }
}

impl DetalleIgvTipoSetter for Detalle {
    fn set_igv_tipo(&mut self, val: &'static str) {
        self.igv_tipo = Some(val);
    }
}
