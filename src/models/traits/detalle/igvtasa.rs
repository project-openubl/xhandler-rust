use crate::models::general::Detalle;

pub trait DetalleIGVTasaGetter {
    fn get_igvtasa(&self) -> &Option<f32>;
}

pub trait DetalleIGVTasaSetter {
    fn set_igvtasa(&mut self, val: f32);
}

impl DetalleIGVTasaGetter for Detalle {
    fn get_igvtasa(&self) -> &Option<f32> {
        &self.igv_tasa
    }
}

impl DetalleIGVTasaSetter for Detalle {
    fn set_igvtasa(&mut self, val: f32) {
        self.igv_tasa = Some(val);
    }
}
