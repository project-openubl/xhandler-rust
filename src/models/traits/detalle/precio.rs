use crate::models::general::Detalle;

pub trait DetallePrecioGetter {
    fn get_precio(&self) -> &Option<f64>;
}

pub trait DetallePrecioSetter {
    fn set_precio(&mut self, val: f64);
}

impl DetallePrecioGetter for Detalle {
    fn get_precio(&self) -> &Option<f64> {
        &self.precio
    }
}

impl DetallePrecioSetter for Detalle {
    fn set_precio(&mut self, val: f64) {
        self.precio = Some(val);
    }
}
