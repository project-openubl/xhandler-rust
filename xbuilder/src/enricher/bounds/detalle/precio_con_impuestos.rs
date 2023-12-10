use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetallePrecioConImpuestosGetter {
    fn get_precio_con_impuestos(&self) -> &Option<Decimal>;
}

pub trait DetallePrecioConImpuestosSetter {
    fn set_precio_con_impuestos(&mut self, val: Decimal);
}

impl DetallePrecioConImpuestosGetter for Detalle {
    fn get_precio_con_impuestos(&self) -> &Option<Decimal> {
        &self.precio_con_impuestos
    }
}

impl DetallePrecioConImpuestosSetter for Detalle {
    fn set_precio_con_impuestos(&mut self, val: Decimal) {
        self.precio_con_impuestos = Some(val);
    }
}
