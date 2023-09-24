use crate::models::general::Detalle;

pub trait DetalleICBTasaGetter {
    fn get_icbtasa(&self) -> &Option<f32>;
}

pub trait DetalleICBTasaSetter {
    fn set_icbtasa(&mut self, val: f32);
}

impl DetalleICBTasaGetter for Detalle {
    fn get_icbtasa(&self) -> &Option<f32> {
        &self.icb_tasa
    }
}

impl DetalleICBTasaSetter for Detalle {
    fn set_icbtasa(&mut self, val: f32) {
        self.icb_tasa = Some(val);
    }
}
