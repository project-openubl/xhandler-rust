use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;

pub trait NoteTipoComprobanteAfectadoGetter {
    fn get_comprobante_afectado_tipo(&self) -> &Option<&'static str>;
}

pub trait NoteTipoComprobanteAfectadoSetter {
    fn set_comprobante_afectado_tipo(&mut self, val: &'static str);
}

impl NoteTipoComprobanteAfectadoGetter for CreditNote {
    fn get_comprobante_afectado_tipo(&self) -> &Option<&'static str> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteTipoComprobanteAfectadoGetter for DebitNote {
    fn get_comprobante_afectado_tipo(&self) -> &Option<&'static str> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteTipoComprobanteAfectadoSetter for CreditNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &'static str) {
        self.comprobante_afectado_tipo = Some(val);
    }
}

impl NoteTipoComprobanteAfectadoSetter for DebitNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &'static str) {
        self.comprobante_afectado_tipo = Some(val);
    }
}
