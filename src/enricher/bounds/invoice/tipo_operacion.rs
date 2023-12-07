use crate::models::invoice::Invoice;

pub trait InvoiceTipoOperacionGetter {
    fn get_tipo_operacion(&self) -> &Option<&'static str>;
}

pub trait InvoiceTipoOperacionSetter {
    fn set_tipo_operacion(&mut self, val: &'static str);
}

impl InvoiceTipoOperacionGetter for Invoice {
    fn get_tipo_operacion(&self) -> &Option<&'static str> {
        &self.tipo_operacion
    }
}

impl InvoiceTipoOperacionSetter for Invoice {
    fn set_tipo_operacion(&mut self, val: &'static str) {
        self.tipo_operacion = Some(val);
    }
}
