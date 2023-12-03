use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleIGVBaseImponibleGetter {
    fn get_igvbaseimponible(&self) -> &Option<Decimal>;
}

pub trait DetalleIGVBaseImponibleSetter {
    fn set_igvbaseimponible(&mut self, val: Decimal);
}

impl DetalleIGVBaseImponibleGetter for Detalle {
    fn get_igvbaseimponible(&self) -> &Option<Decimal> {
        &self.igv_base_imponible
    }
}

impl DetalleIGVBaseImponibleSetter for Detalle {
    fn set_igvbaseimponible(&mut self, val: Decimal) {
        self.igv_base_imponible = Some(val);
    }
}
