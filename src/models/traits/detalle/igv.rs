use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleIGVGetter {
    fn get_igv(&self) -> &Option<Decimal>;
}

pub trait DetalleIGVSetter {
    fn set_igv(&mut self, val: Decimal);
}

impl DetalleIGVGetter for Detalle {
    fn get_igv(&self) -> &Option<Decimal> {
        &self.igv
    }
}

impl DetalleIGVSetter for Detalle {
    fn set_igv(&mut self, val: Decimal) {
        self.igv = Some(val);
    }
}
