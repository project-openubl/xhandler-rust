use crate::models::common::FormaDePago;
use crate::models::invoice::Invoice;

pub trait InvoiceFormaDePagoGetter {
    fn get_forma_de_pago(&self) -> &Option<FormaDePago>;
}

pub trait InvoiceFormaDePagoSetter {
    fn set_forma_de_pago(&mut self, val: FormaDePago);
}

impl InvoiceFormaDePagoGetter for Invoice {
    fn get_forma_de_pago(&self) -> &Option<FormaDePago> {
        &self.forma_de_pago
    }
}

impl InvoiceFormaDePagoSetter for Invoice {
    fn set_forma_de_pago(&mut self, val: FormaDePago) {
        self.forma_de_pago = Some(val);
    }
}
