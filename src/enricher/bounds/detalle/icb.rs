use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIcbGetter {
    fn get_icb(&self) -> &Option<Decimal>;
}

pub trait DetalleIcbSetter {
    fn set_icb(&mut self, val: Decimal);
}

impl DetalleIcbGetter for Detalle {
    fn get_icb(&self) -> &Option<Decimal> {
        &self.icb
    }
}

impl DetalleIcbSetter for Detalle {
    fn set_icb(&mut self, val: Decimal) {
        self.icb = Some(val);
    }
}
