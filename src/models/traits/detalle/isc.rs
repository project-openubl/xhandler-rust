use crate::models::general::Detalle;

pub trait DetalleISCGetter {
    fn get_isc(&self) -> &Option<f64>;
}

pub trait DetalleISCSetter {
    fn set_isc(&mut self, val: f64);
}

impl DetalleISCGetter for Detalle {
    fn get_isc(&self) -> &Option<f64> {
        &self.isc
    }
}

impl DetalleISCSetter for Detalle {
    fn set_isc(&mut self, val: f64) {
        self.isc = Some(val);
    }
}
