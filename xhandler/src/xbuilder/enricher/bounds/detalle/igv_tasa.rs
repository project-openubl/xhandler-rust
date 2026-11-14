use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detalle;

pub trait DetalleIgvTasaGetter {
    fn igv_tasa(&self) -> &Option<Decimal>;
}

pub trait DetalleIgvTasaSetter {
    fn set_igv_tasa(&mut self, val: Decimal);
}

impl DetalleIgvTasaGetter for Detalle {
    fn igv_tasa(&self) -> &Option<Decimal> {
        &self.igv_tasa
    }
}

impl DetalleIgvTasaSetter for Detalle {
    fn set_igv_tasa(&mut self, val: Decimal) {
        self.igv_tasa = Some(val);
    }
}
