use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleISCGetter {
    fn get_isc(&self) -> &Option<Decimal>;
}

pub trait DetalleISCSetter {
    fn set_isc(&mut self, val: Decimal);
}

impl DetalleISCGetter for Detalle {
    fn get_isc(&self) -> &Option<Decimal> {
        &self.isc
    }
}

impl DetalleISCSetter for Detalle {
    fn set_isc(&mut self, val: Decimal) {
        self.isc = Some(val);
    }
}
