use crate::models::general::FormaDePago;
use crate::models::invoice::Invoice;

pub trait InvoiceFormaDePagoGetter {
    fn get_formadepago(&self) -> &Option<FormaDePago>;
}

pub trait InvoiceFormaDePagoSetter {
    fn set_formadepago(&mut self, val: FormaDePago);
}

impl InvoiceFormaDePagoGetter for Invoice {
    fn get_formadepago(&self) -> &Option<FormaDePago> {
        &self.forma_de_pago
    }
}

impl InvoiceFormaDePagoSetter for Invoice {
    fn set_formadepago(&mut self, val: FormaDePago) {
        self.forma_de_pago = Some(val);
    }
}
