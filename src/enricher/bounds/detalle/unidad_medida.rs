use crate::models::common::Detalle;

pub trait DetalleUnidadMedidaGetter {
    fn get_unidad_medida(&self) -> &Option<&'static str>;
}

pub trait DetalleUnidadMedidaSetter {
    fn set_unidad_medida(&mut self, val: &'static str);
}

impl DetalleUnidadMedidaGetter for Detalle {
    fn get_unidad_medida(&self) -> &Option<&'static str> {
        &self.unidad_medida
    }
}

impl DetalleUnidadMedidaSetter for Detalle {
    fn set_unidad_medida(&mut self, val: &'static str) {
        self.unidad_medida = Some(val);
    }
}
