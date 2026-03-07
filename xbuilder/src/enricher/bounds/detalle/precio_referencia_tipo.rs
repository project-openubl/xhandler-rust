use crate::models::common::Detalle;

pub trait DetallePrecioReferenciaTipoGetter {
    fn get_precio_referencia_tipo(&self) -> &Option<String>;
}

pub trait DetallePrecioReferenciaTipoSetter {
    fn set_precio_referencia_tipo(&mut self, val: &str);
}

impl DetallePrecioReferenciaTipoGetter for Detalle {
    fn get_precio_referencia_tipo(&self) -> &Option<String> {
        &self.precio_referencia_tipo
    }
}

impl DetallePrecioReferenciaTipoSetter for Detalle {
    fn set_precio_referencia_tipo(&mut self, val: &str) {
        self.precio_referencia_tipo = Some(val.to_string());
    }
}
