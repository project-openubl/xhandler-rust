use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIcbTasaGetter {
    fn get_icb_tasa(&self) -> &Option<Decimal>;
}

pub trait DetalleIcbTasaSetter {
    fn set_icb_tasa(&mut self, val: Decimal);
}

impl DetalleIcbTasaGetter for Detalle {
    fn get_icb_tasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl DetalleIcbTasaSetter for Detalle {
    fn set_icb_tasa(&mut self, val: Decimal) {
        self.icb_tasa = Some(val);
    }
}
