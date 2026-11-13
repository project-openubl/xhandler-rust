use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIgvBaseImponibleGetter {
    fn get_igv_base_imponible(&self) -> &Option<Decimal>;
}

pub trait DetalleIgvBaseImponibleSetter {
    fn set_igv_base_imponible(&mut self, val: Decimal);
}

impl DetalleIgvBaseImponibleGetter for Detalle {
    fn get_igv_base_imponible(&self) -> &Option<Decimal> {
        &self.igv_base_imponible
    }
}

impl DetalleIgvBaseImponibleSetter for Detalle {
    fn set_igv_base_imponible(&mut self, val: Decimal) {
        self.igv_base_imponible = Some(val);
    }
}
