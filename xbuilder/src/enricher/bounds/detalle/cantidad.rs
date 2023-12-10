use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleCantidadGetter {
    fn get_cantidad(&self) -> Decimal;
}

pub trait DetalleCantidadSetter {
    fn set_cantidad(&mut self, val: Decimal);
}

impl DetalleCantidadGetter for Detalle {
    fn get_cantidad(&self) -> Decimal {
        self.cantidad
    }
}

impl DetalleCantidadSetter for Detalle {
    fn set_cantidad(&mut self, val: Decimal) {
        self.cantidad = val;
    }
}
