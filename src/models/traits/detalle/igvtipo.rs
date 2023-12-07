use crate::models::common::Detalle;

pub trait DetalleIGVTipoGetter {
    fn get_igvtipo(&self) -> &Option<&'static str>;
}

pub trait DetalleIGVTipoSetter {
    fn set_igvtipo(&mut self, val: &'static str);
}

impl DetalleIGVTipoGetter for Detalle {
    fn get_igvtipo(&self) -> &Option<&'static str> {
        &self.igv_tipo
    }
}

impl DetalleIGVTipoSetter for Detalle {
    fn set_igvtipo(&mut self, val: &'static str) {
        self.igv_tipo = Some(val);
    }
}
