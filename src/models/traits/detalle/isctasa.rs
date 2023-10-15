use crate::models::general::Detalle;

pub trait DetalleISCTasaGetter {
    fn get_isctasa(&self) -> &Option<f64>;
}

pub trait DetalleISCTasaSetter {
    fn set_isctasa(&mut self, val: f64);
}

impl DetalleISCTasaGetter for Detalle {
    fn get_isctasa(&self) -> &Option<f64> {
        &self.isc_tasa
    }
}

impl DetalleISCTasaSetter for Detalle {
    fn set_isctasa(&mut self, val: f64) {
        self.isc_tasa = Some(val);
    }
}
