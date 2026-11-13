use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleCantidadGetter {
    fn cantidad(&self) -> Decimal;
}

impl DetalleCantidadGetter for Detalle {
    fn cantidad(&self) -> Decimal {
        self.cantidad
    }
}
