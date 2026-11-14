use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detalle;

pub trait DetalleIscGetter {
    fn isc(&self) -> &Option<Decimal>;
}

pub trait DetalleIscSetter {
    fn set_isc(&mut self, val: Decimal);
}

impl DetalleIscGetter for Detalle {
    fn isc(&self) -> &Option<Decimal> {
        &self.isc
    }
}

impl DetalleIscSetter for Detalle {
    fn set_isc(&mut self, val: Decimal) {
        self.isc = Some(val);
    }
}
