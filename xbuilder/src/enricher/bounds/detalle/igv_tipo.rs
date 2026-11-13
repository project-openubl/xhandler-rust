use crate::models::common::Detalle;

pub trait DetalleIgvTipoGetter {
    fn igv_tipo(&self) -> &Option<String>;
}

pub trait DetalleIgvTipoSetter {
    fn set_igv_tipo(&mut self, val: &str);
}

impl DetalleIgvTipoGetter for Detalle {
    fn igv_tipo(&self) -> &Option<String> {
        &self.igv_tipo
    }
}

impl DetalleIgvTipoSetter for Detalle {
    fn set_igv_tipo(&mut self, val: &str) {
        self.igv_tipo = Some(val.to_string());
    }
}
