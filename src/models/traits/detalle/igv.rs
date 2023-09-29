use crate::models::general::Detalle;

pub trait DetalleIGVGetter {
    fn get_igv(&self) -> &Option<f64>;
}

pub trait DetalleIGVSetter {
    fn set_igv(&mut self, val: f64);
}

impl DetalleIGVGetter for Detalle {
    fn get_igv(&self) -> &Option<f64> {
        &self.igv
    }
}

impl DetalleIGVSetter for Detalle {
    fn set_igv(&mut self, val: f64) {
        self.igv = Some(val);
    }
}
