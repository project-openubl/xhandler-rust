use crate::models::invoice::Invoice;

pub trait InvoiceTipoComprobanteGetter {
    fn get_tipocomprobante(&self) -> &Option<&'static str>;
}

pub trait InvoiceTipoComprobanteSetter {
    fn set_tipocomprobante(&mut self, val: &'static str);
}

impl InvoiceTipoComprobanteGetter for Invoice {
    fn get_tipocomprobante(&self) -> &Option<&'static str> {
        &self.tipo_comprobante
    }
}

impl InvoiceTipoComprobanteSetter for Invoice {
    fn set_tipocomprobante(&mut self, val: &'static str) {
        self.tipo_comprobante = Some(val);
    }
}
