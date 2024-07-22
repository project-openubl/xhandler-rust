use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleCantidadGetter {
    fn get_cantidad(&self) -> Decimal;
}

impl DetalleCantidadGetter for Detalle {
    fn get_cantidad(&self) -> Decimal {
        self.cantidad
    }
}
