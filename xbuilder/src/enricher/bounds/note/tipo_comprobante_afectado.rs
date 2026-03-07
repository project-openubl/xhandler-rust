use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;

pub trait NoteTipoComprobanteAfectadoGetter {
    fn get_comprobante_afectado_tipo(&self) -> &Option<String>;
}

pub trait NoteTipoComprobanteAfectadoSetter {
    fn set_comprobante_afectado_tipo(&mut self, val: &str);
}

impl NoteTipoComprobanteAfectadoGetter for CreditNote {
    fn get_comprobante_afectado_tipo(&self) -> &Option<String> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteTipoComprobanteAfectadoGetter for DebitNote {
    fn get_comprobante_afectado_tipo(&self) -> &Option<String> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteTipoComprobanteAfectadoSetter for CreditNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &str) {
        self.comprobante_afectado_tipo = Some(val.to_string());
    }
}

impl NoteTipoComprobanteAfectadoSetter for DebitNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &str) {
        self.comprobante_afectado_tipo = Some(val.to_string());
    }
}
