use crate::models::general::Detalle;

pub trait DetalleISCBaseImponibleGetter {
    fn get_iscbaseimponible(&self) -> &Option<f64>;
}

pub trait DetalleISCBaseImponibleSetter {
    fn set_iscbaseimponible(&mut self, val: f64);
}

impl DetalleISCBaseImponibleGetter for Detalle {
    fn get_iscbaseimponible(&self) -> &Option<f64> {
        &self.isc_base_imponible
    }
}

impl DetalleISCBaseImponibleSetter for Detalle {
    fn set_iscbaseimponible(&mut self, val: f64) {
        self.isc_base_imponible = Some(val);
    }
}
