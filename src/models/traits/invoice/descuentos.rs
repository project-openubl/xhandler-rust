use rust_decimal::Decimal;

use crate::models::common::Descuento;
use crate::models::invoice::Invoice;

pub trait InvoiceDescuentosGetter {
    fn get_descuentos(&mut self) -> &mut Vec<Descuento>;
}

impl InvoiceDescuentosGetter for Invoice {
    fn get_descuentos(&mut self) -> &mut Vec<Descuento> {
        &mut self.descuentos
    }
}

//

pub trait DescuentoGetter {
    fn get_tipo(&self) -> &Option<&'static str>;
    fn get_monto(&self) -> Decimal;
    fn get_monto_base(&self) -> &Option<Decimal>;
    fn get_factor(&self) -> &Option<Decimal>;
}

pub trait DescuentoSetter {
    fn set_tipo(&mut self, val: &'static str);
    fn set_monto_base(&mut self, val: Decimal);
    fn set_factor(&mut self, val: Decimal);
}

impl DescuentoGetter for Descuento {
    fn get_tipo(&self) -> &Option<&'static str> {
        &self.tipo
    }

    fn get_monto(&self) -> Decimal {
        self.monto
    }

    fn get_monto_base(&self) -> &Option<Decimal> {
        &self.monto_base
    }

    fn get_factor(&self) -> &Option<Decimal> {
        &self.factor
    }
}

impl DescuentoSetter for Descuento {
    fn set_tipo(&mut self, val: &'static str) {
        self.tipo = Some(val);
    }

    fn set_monto_base(&mut self, val: Decimal) {
        self.monto_base = Some(val);
    }

    fn set_factor(&mut self, val: Decimal) {
        self.factor = Some(val);
    }
}
