use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleTotalImpuestosGetter {
    fn get_totalimpuestos(&self) -> &Option<Decimal>;
}

pub trait DetalleTotalImpuestosSetter {
    fn set_totalimpuestos(&mut self, val: Decimal);
}

impl DetalleTotalImpuestosGetter for Detalle {
    fn get_totalimpuestos(&self) -> &Option<Decimal> {
        &self.total_impuestos
    }
}

impl DetalleTotalImpuestosSetter for Detalle {
    fn set_totalimpuestos(&mut self, val: Decimal) {
        self.total_impuestos = Some(val);
    }
}
