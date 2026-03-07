use crate::models::invoice::Invoice;

pub trait InvoiceTipoOperacionGetter {
    fn get_tipo_operacion(&self) -> &Option<String>;
}

pub trait InvoiceTipoOperacionSetter {
    fn set_tipo_operacion(&mut self, val: &str);
}

impl InvoiceTipoOperacionGetter for Invoice {
    fn get_tipo_operacion(&self) -> &Option<String> {
        &self.tipo_operacion
    }
}

impl InvoiceTipoOperacionSetter for Invoice {
    fn set_tipo_operacion(&mut self, val: &str) {
        self.tipo_operacion = Some(val.to_string());
    }
}
