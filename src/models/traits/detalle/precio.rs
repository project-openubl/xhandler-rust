use crate::models::general::Detalle;
use rust_decimal::Decimal;

pub trait DetallePrecioGetter {
    fn get_precio(&self) -> &Option<Decimal>;
}

pub trait DetallePrecioSetter {
    fn set_precio(&mut self, val: Decimal);
}

impl DetallePrecioGetter for Detalle {
    fn get_precio(&self) -> &Option<Decimal> {
        &self.precio
    }
}

impl DetallePrecioSetter for Detalle {
    fn set_precio(&mut self, val: Decimal) {
        self.precio = Some(val);
    }
}
