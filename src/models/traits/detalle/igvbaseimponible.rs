use crate::models::general::Detalle;

pub trait DetalleIGVBaseImponibleGetter {
    fn get_igvbaseimponible(&self) -> &Option<f64>;
}

pub trait DetalleIGVBaseImponibleSetter {
    fn set_igvbaseimponible(&mut self, val: f64);
}

impl DetalleIGVBaseImponibleGetter for Detalle {
    fn get_igvbaseimponible(&self) -> &Option<f64> {
        &self.igv_base_imponible
    }
}

impl DetalleIGVBaseImponibleSetter for Detalle {
    fn set_igvbaseimponible(&mut self, val: f64) {
        self.igv_base_imponible = Some(val);
    }
}
