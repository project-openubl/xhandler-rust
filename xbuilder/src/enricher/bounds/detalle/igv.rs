use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleIgvGetter {
    fn get_igv(&self) -> &Option<Decimal>;
}

pub trait DetalleIgvSetter {
    fn set_igv(&mut self, val: Decimal);
}

impl DetalleIgvGetter for Detalle {
    fn get_igv(&self) -> &Option<Decimal> {
        &self.igv
    }
}

impl DetalleIgvSetter for Detalle {
    fn set_igv(&mut self, val: Decimal) {
        self.igv = Some(val);
    }
}
