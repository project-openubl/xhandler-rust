use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detalle;

pub trait DetalleIscBaseImponibleGetter {
    fn isc_base_imponible(&self) -> &Option<Decimal>;
}

pub trait DetalleIscBaseImponibleSetter {
    fn set_isc_base_imponible(&mut self, val: Decimal);
}

impl DetalleIscBaseImponibleGetter for Detalle {
    fn isc_base_imponible(&self) -> &Option<Decimal> {
        &self.isc_base_imponible
    }
}

impl DetalleIscBaseImponibleSetter for Detalle {
    fn set_isc_base_imponible(&mut self, val: Decimal) {
        self.isc_base_imponible = Some(val);
    }
}
