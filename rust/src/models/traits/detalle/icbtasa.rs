use crate::models::general::Detalle;

pub trait ICBTasaGetter {
    fn get_icbtasa(&self) -> &Option<f64>;
}

pub trait ICBTasaSetter {
    fn set_icbtasa(&mut self, val: f64);
}

impl ICBTasaGetter for Detalle {
    fn get_icbtasa(&self) -> &Option<f64> {
        &self.icb_tasa
    }
}

impl ICBTasaSetter for Detalle {
    fn set_icbtasa(&mut self, val: f64) {
        self.icb_tasa = Some(val);
    }
}
