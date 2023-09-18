use crate::models::general::FormaDePago;
use crate::models::invoice::Invoice;

pub trait FormaDePagoGetter {
    fn get_formadepago(&self) -> &Option<FormaDePago>;
}

pub trait FormaDePagoSetter {
    fn set_formadepago(&mut self, val: FormaDePago);
}

impl FormaDePagoGetter for Invoice {
    fn get_formadepago(&self) -> &Option<FormaDePago> {
        &self.forma_de_pago
    }
}

impl FormaDePagoSetter for Invoice {
    fn set_formadepago(&mut self, val: FormaDePago) {
        self.forma_de_pago = Some(val);
    }
}
