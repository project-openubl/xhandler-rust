use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIscBaseImponibleGetter {
    fn get_isc_base_imponible(&self) -> &Option<Decimal>;
}

pub trait DetalleISCBaseImponibleSetter {
    fn set_isc_base_imponible(&mut self, val: Decimal);
}

impl DetalleIscBaseImponibleGetter for Detalle {
    fn get_isc_base_imponible(&self) -> &Option<Decimal> {
        &self.isc_base_imponible
    }
}

impl DetalleISCBaseImponibleSetter for Detalle {
    fn set_isc_base_imponible(&mut self, val: Decimal) {
        self.isc_base_imponible = Some(val);
    }
}
