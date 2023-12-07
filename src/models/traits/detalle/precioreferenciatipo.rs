use crate::models::common::Detalle;

pub trait DetallePrecioReferenciaTipoGetter {
    fn get_precioreferenciatipo(&self) -> &Option<&'static str>;
}

pub trait DetallePrecioReferenciaTipoSetter {
    fn set_precioreferenciatipo(&mut self, val: &'static str);
}

impl DetallePrecioReferenciaTipoGetter for Detalle {
    fn get_precioreferenciatipo(&self) -> &Option<&'static str> {
        &self.precio_referencia_tipo
    }
}

impl DetallePrecioReferenciaTipoSetter for Detalle {
    fn set_precioreferenciatipo(&mut self, val: &'static str) {
        self.precio_referencia_tipo = Some(val);
    }
}
