use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIGVTasaGetter {
    fn get_igvtasa(&self) -> &Option<Decimal>;
}

pub trait DetalleIGVTasaSetter {
    fn set_igvtasa(&mut self, val: Decimal);
}

impl DetalleIGVTasaGetter for Detalle {
    fn get_igvtasa(&self) -> &Option<Decimal> {
        &self.igv_tasa
    }
}

impl DetalleIGVTasaSetter for Detalle {
    fn set_igvtasa(&mut self, val: Decimal) {
        self.igv_tasa = Some(val);
    }
}
