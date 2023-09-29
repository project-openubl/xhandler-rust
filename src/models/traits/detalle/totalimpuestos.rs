use crate::models::general::Detalle;

pub trait DetalleTotalImpuestosGetter {
    fn get_totalimpuestos(&self) -> &Option<f64>;
}

pub trait DetalleTotalImpuestosSetter {
    fn set_totalimpuestos(&mut self, val: f64);
}

impl DetalleTotalImpuestosGetter for Detalle {
    fn get_totalimpuestos(&self) -> &Option<f64> {
        &self.total_impuestos
    }
}

impl DetalleTotalImpuestosSetter for Detalle {
    fn set_totalimpuestos(&mut self, val: f64) {
        self.total_impuestos = Some(val);
    }
}
