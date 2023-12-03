use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleISCTasaGetter {
    fn get_isctasa(&self) -> &Option<Decimal>;
}

pub trait DetalleISCTasaSetter {
    fn set_isctasa(&mut self, val: Decimal);
}

impl DetalleISCTasaGetter for Detalle {
    fn get_isctasa(&self) -> &Option<Decimal> {
        &self.isc_tasa
    }
}

impl DetalleISCTasaSetter for Detalle {
    fn set_isctasa(&mut self, val: Decimal) {
        self.isc_tasa = Some(val);
    }
}
