use crate::models::general::Detalle;

pub trait DetalleIGVTasaGetter {
    fn get_igvtasa(&self) -> &Option<f64>;
}

pub trait DetalleIGVTasaSetter {
    fn set_igvtasa(&mut self, val: f64);
}

impl DetalleIGVTasaGetter for Detalle {
    fn get_igvtasa(&self) -> &Option<f64> {
        &self.igv_tasa
    }
}

impl DetalleIGVTasaSetter for Detalle {
    fn set_igvtasa(&mut self, val: f64) {
        self.igv_tasa = Some(val);
    }
}
