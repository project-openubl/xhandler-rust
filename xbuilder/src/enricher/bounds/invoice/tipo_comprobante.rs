use crate::models::invoice::Invoice;

pub trait InvoiceTipoComprobanteGetter {
    fn get_tipo_comprobante(&self) -> &Option<String>;
}

pub trait InvoiceTipoComprobanteSetter {
    fn set_tipo_comprobante(&mut self, val: &str);
}

impl InvoiceTipoComprobanteGetter for Invoice {
    fn get_tipo_comprobante(&self) -> &Option<String> {
        &self.tipo_comprobante
    }
}

impl InvoiceTipoComprobanteSetter for Invoice {
    fn set_tipo_comprobante(&mut self, val: &str) {
        self.tipo_comprobante = Some(val.to_string());
    }
}
