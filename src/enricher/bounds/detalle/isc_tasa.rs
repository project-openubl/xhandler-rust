use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIscTasaGetter {
    fn get_isc_tasa(&self) -> &Option<Decimal>;
}

pub trait DetalleIscTasaSetter {
    fn set_isc_tasa(&mut self, val: Decimal);
}

impl DetalleIscTasaGetter for Detalle {
    fn get_isc_tasa(&self) -> &Option<Decimal> {
        &self.isc_tasa
    }
}

impl DetalleIscTasaSetter for Detalle {
    fn set_isc_tasa(&mut self, val: Decimal) {
        self.isc_tasa = Some(val);
    }
}
