use crate::models::invoice::Invoice;

pub trait TipoComprobanteGetter {
    fn get_tipocomprobante(&self) -> &Option<&'static str>;
}

pub trait TipoComprobanteSetter {
    fn set_tipocomprobante(&mut self, val: &'static str);
}

impl TipoComprobanteGetter for Invoice {
    fn get_tipocomprobante(&self) -> &Option<&'static str> {
        &self.tipo_comprobante
    }
}

impl TipoComprobanteSetter for Invoice {
    fn set_tipocomprobante(&mut self, val: &'static str) {
        self.tipo_comprobante = Some(val);
    }
}
