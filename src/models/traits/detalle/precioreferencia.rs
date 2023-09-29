use crate::models::general::Detalle;

pub trait DetallePrecioReferenciaGetter {
    fn get_precioreferencia(&self) -> &Option<f64>;
}

pub trait DetallePrecioReferenciaSetter {
    fn set_precioreferencia(&mut self, val: f64);
}

impl DetallePrecioReferenciaGetter for Detalle {
    fn get_precioreferencia(&self) -> &Option<f64> {
        &self.precio_referencia
    }
}

impl DetallePrecioReferenciaSetter for Detalle {
    fn set_precioreferencia(&mut self, val: f64) {
        self.precio_referencia = Some(val);
    }
}
