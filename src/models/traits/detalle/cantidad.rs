use crate::models::general::Detalle;

pub trait DetalleCantidadGetter {
    fn get_cantidad(&self) -> &f64;
}

pub trait DetalleCantidadSetter {
    fn set_cantidad(&mut self, val: f64);
}

impl DetalleCantidadGetter for Detalle {
    fn get_cantidad(&self) -> &f64 {
        &self.cantidad
    }
}

impl DetalleCantidadSetter for Detalle {
    fn set_cantidad(&mut self, val: f64) {
        self.cantidad = val;
    }
}
