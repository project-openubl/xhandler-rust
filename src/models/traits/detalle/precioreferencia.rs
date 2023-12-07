use rust_decimal::Decimal;

use crate::models::common::Detalle;

pub trait DetallePrecioReferenciaGetter {
    fn get_precioreferencia(&self) -> &Option<Decimal>;
}

pub trait DetallePrecioReferenciaSetter {
    fn set_precioreferencia(&mut self, val: Decimal);
}

impl DetallePrecioReferenciaGetter for Detalle {
    fn get_precioreferencia(&self) -> &Option<Decimal> {
        &self.precio_referencia
    }
}

impl DetallePrecioReferenciaSetter for Detalle {
    fn set_precioreferencia(&mut self, val: Decimal) {
        self.precio_referencia = Some(val);
    }
}
