use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleICBTasaGetter {
    fn get_icbtasa(&self) -> &Option<Decimal>;
}

pub trait DetalleICBTasaSetter {
    fn set_icbtasa(&mut self, val: Decimal);
}

impl DetalleICBTasaGetter for Detalle {
    fn get_icbtasa(&self) -> &Option<Decimal> {
        &self.icb_tasa
    }
}

impl DetalleICBTasaSetter for Detalle {
    fn set_icbtasa(&mut self, val: Decimal) {
        self.icb_tasa = Some(val);
    }
}
