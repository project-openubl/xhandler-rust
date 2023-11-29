use crate::models::general::Detalle;
use rust_decimal::Decimal;

pub trait DetalleISCBaseImponibleGetter {
    fn get_iscbaseimponible(&self) -> &Option<Decimal>;
}

pub trait DetalleISCBaseImponibleSetter {
    fn set_iscbaseimponible(&mut self, val: Decimal);
}

impl DetalleISCBaseImponibleGetter for Detalle {
    fn get_iscbaseimponible(&self) -> &Option<Decimal> {
        &self.isc_base_imponible
    }
}

impl DetalleISCBaseImponibleSetter for Detalle {
    fn set_iscbaseimponible(&mut self, val: Decimal) {
        self.isc_base_imponible = Some(val);
    }
}
