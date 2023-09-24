use crate::models::general::Detalle;

pub trait DetalleISCTasaGetter {
    fn get_isctasa(&self) -> &Option<f32>;
}

pub trait DetalleISCTasaSetter {
    fn set_isctasa(&mut self, val: f32);
}

impl DetalleISCTasaGetter for Detalle {
    fn get_isctasa(&self) -> &Option<f32> {
        &self.isc_tasa
    }
}

impl DetalleISCTasaSetter for Detalle {
    fn set_isctasa(&mut self, val: f32) {
        self.isc_tasa = Some(val);
    }
}
