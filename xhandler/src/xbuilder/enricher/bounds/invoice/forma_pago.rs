use crate::xbuilder::models::common::FormaPago;
use crate::xbuilder::models::invoice::Invoice;

pub trait InvoiceFormaPagoGetter {
    fn forma_pago(&self) -> &Option<FormaPago>;
}

pub trait InvoiceFormaPagoSetter {
    fn set_forma_pago(&mut self, val: FormaPago);
}

impl InvoiceFormaPagoGetter for Invoice {
    fn forma_pago(&self) -> &Option<FormaPago> {
        &self.forma_pago
    }
}

impl InvoiceFormaPagoSetter for Invoice {
    fn set_forma_pago(&mut self, val: FormaPago) {
        self.forma_pago = Some(val);
    }
}
