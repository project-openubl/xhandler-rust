use crate::models::general::Detalle;

pub trait UnidadMedidaGetter {
    fn get_unidadmedida(&self) -> &Option<&'static str>;
}

pub trait UnidadMedidaSetter {
    fn set_unidadmedida(&mut self, val: &'static str);
}

impl UnidadMedidaGetter for Detalle {
    fn get_unidadmedida(&self) -> &Option<&'static str> {
        &self.unidad_medida
    }
}

impl UnidadMedidaSetter for Detalle {
    fn set_unidadmedida(&mut self, val: &'static str) {
        self.unidad_medida = Some(val);
    }
}
