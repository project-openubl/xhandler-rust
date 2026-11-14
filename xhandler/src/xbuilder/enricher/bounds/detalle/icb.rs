use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detalle;

pub trait DetalleIcbGetter {
    fn icb(&self) -> &Option<Decimal>;
}

pub trait DetalleIcbSetter {
    fn set_icb(&mut self, val: Decimal);
}

impl DetalleIcbGetter for Detalle {
    fn icb(&self) -> &Option<Decimal> {
        &self.icb
    }
}

impl DetalleIcbSetter for Detalle {
    fn set_icb(&mut self, val: Decimal) {
        self.icb = Some(val);
    }
}
