use crate::models::general::Descuento;
use crate::models::invoice::Invoice;

pub trait DescuentosGetter {
    fn get_descuentos(&mut self) -> &mut Vec<Descuento>;
}

impl DescuentosGetter for Invoice {
    fn get_descuentos(&mut self) -> &mut Vec<Descuento> {
        &mut self.descuentos
    }
}

//

pub trait DescuentoGetter {
    fn get_tipo(&self) -> &Option<&'static str>;
    fn get_monto(&self) -> f64;
    fn get_monto_base(&self) -> &Option<f64>;
    fn get_factor(&self) -> &Option<f64>;
}

pub trait DescuentoSetter {
    fn set_tipo(&mut self, val: &'static str);
    fn set_monto_base(&mut self, val: f64);
    fn set_factor(&mut self, val: f64);
}

impl DescuentoGetter for Descuento {
    fn get_tipo(&self) -> &Option<&'static str> {
        &self.tipo
    }

    fn get_monto(&self) -> f64 {
        self.monto
    }

    fn get_monto_base(&self) -> &Option<f64> {
        &self.monto_base
    }

    fn get_factor(&self) -> &Option<f64> {
        &self.factor
    }
}

impl DescuentoSetter for Descuento {
    fn set_tipo(&mut self, val: &'static str) {
        self.tipo = Some(val);
    }

    fn set_monto_base(&mut self, val: f64) {
        self.monto_base = Some(val);
    }

    fn set_factor(&mut self, val: f64) {
        self.factor = Some(val);
    }
}
