use crate::models::general::Detalle;

pub trait DetalleICBGetter {
    fn get_icb(&self) -> &Option<f64>;
}

pub trait DetalleICBSetter {
    fn set_icb(&mut self, val: f64);
}

impl DetalleICBGetter for Detalle {
    fn get_icb(&self) -> &Option<f64> {
        &self.icb
    }
}

impl DetalleICBSetter for Detalle {
    fn set_icb(&mut self, val: f64) {
        self.icb = Some(val);
    }
}
