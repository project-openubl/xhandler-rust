use crate::models::general::Detalle;

pub trait DetallePrecioConImpuestosGetter {
    fn get_precioconimpuestos(&self) -> &Option<f64>;
}

pub trait DetallePrecioConImpuestosSetter {
    fn set_precioconimpuestos(&mut self, val: f64);
}

impl DetallePrecioConImpuestosGetter for Detalle {
    fn get_precioconimpuestos(&self) -> &Option<f64> {
        &self.precio_con_impuestos
    }
}

impl DetallePrecioConImpuestosSetter for Detalle {
    fn set_precioconimpuestos(&mut self, val: f64) {
        self.precio_con_impuestos = Some(val);
    }
}
