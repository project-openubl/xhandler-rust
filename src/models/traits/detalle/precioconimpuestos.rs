use crate::models::general::Detalle;
use rust_decimal::Decimal;

pub trait DetallePrecioConImpuestosGetter {
    fn get_precioconimpuestos(&self) -> &Option<Decimal>;
}

pub trait DetallePrecioConImpuestosSetter {
    fn set_precioconimpuestos(&mut self, val: Decimal);
}

impl DetallePrecioConImpuestosGetter for Detalle {
    fn get_precioconimpuestos(&self) -> &Option<Decimal> {
        &self.precio_con_impuestos
    }
}

impl DetallePrecioConImpuestosSetter for Detalle {
    fn set_precioconimpuestos(&mut self, val: Decimal) {
        self.precio_con_impuestos = Some(val);
    }
}
