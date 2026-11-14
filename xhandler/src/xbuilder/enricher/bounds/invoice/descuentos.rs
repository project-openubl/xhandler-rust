use rust_decimal::Decimal;

use crate::xbuilder::models::common::Descuento;
use crate::xbuilder::models::invoice::Invoice;

pub trait InvoiceDescuentosGetter {
    fn descuentos(&self) -> &[Descuento];
    fn descuentos_mut(&mut self) -> &mut Vec<Descuento>;
}

impl InvoiceDescuentosGetter for Invoice {
    fn descuentos(&self) -> &[Descuento] {
        &self.descuentos
    }

    fn descuentos_mut(&mut self) -> &mut Vec<Descuento> {
        &mut self.descuentos
    }
}

//

pub trait DescuentoGetter {
    fn tipo(&self) -> &Option<String>;
    fn monto(&self) -> Decimal;
    fn monto_base(&self) -> &Option<Decimal>;
    fn factor(&self) -> &Option<Decimal>;
}

pub trait DescuentoSetter {
    fn set_tipo(&mut self, val: &str);
    fn set_monto_base(&mut self, val: Decimal);
    fn set_factor(&mut self, val: Decimal);
}

impl DescuentoGetter for Descuento {
    fn tipo(&self) -> &Option<String> {
        &self.tipo
    }

    fn monto(&self) -> Decimal {
        self.monto
    }

    fn monto_base(&self) -> &Option<Decimal> {
        &self.monto_base
    }

    fn factor(&self) -> &Option<Decimal> {
        &self.factor
    }
}

impl DescuentoSetter for Descuento {
    fn set_tipo(&mut self, val: &str) {
        self.tipo = Some(val.to_string());
    }

    fn set_monto_base(&mut self, val: Decimal) {
        self.monto_base = Some(val);
    }

    fn set_factor(&mut self, val: Decimal) {
        self.factor = Some(val);
    }
}
