use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetalleTotalImpuestosGetter {
    fn get_total_impuestos(&self) -> &Option<Decimal>;
}

pub trait DetalleTotalImpuestosSetter {
    fn set_total_impuestos(&mut self, val: Decimal);
}

impl DetalleTotalImpuestosGetter for Detalle {
    fn get_total_impuestos(&self) -> &Option<Decimal> {
        &self.total_impuestos
    }
}

impl DetalleTotalImpuestosSetter for Detalle {
    fn set_total_impuestos(&mut self, val: Decimal) {
        self.total_impuestos = Some(val);
    }
}
