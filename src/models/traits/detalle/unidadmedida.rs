use crate::models::general::Detalle;

pub trait DetalleUnidadMedidaGetter {
    fn get_unidadmedida(&self) -> &Option<&'static str>;
}

pub trait DetalleUnidadMedidaSetter {
    fn set_unidadmedida(&mut self, val: &'static str);
}

impl DetalleUnidadMedidaGetter for Detalle {
    fn get_unidadmedida(&self) -> &Option<&'static str> {
        &self.unidad_medida
    }
}

impl DetalleUnidadMedidaSetter for Detalle {
    fn set_unidadmedida(&mut self, val: &'static str) {
        self.unidad_medida = Some(val);
    }
}
