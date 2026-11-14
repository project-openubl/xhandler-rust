use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detalle;

pub trait DetalleIgvGetter {
    fn igv(&self) -> &Option<Decimal>;
}

pub trait DetalleIgvSetter {
    fn set_igv(&mut self, val: Decimal);
}

impl DetalleIgvGetter for Detalle {
    fn igv(&self) -> &Option<Decimal> {
        &self.igv
    }
}

impl DetalleIgvSetter for Detalle {
    fn set_igv(&mut self, val: Decimal) {
        self.igv = Some(val);
    }
}
