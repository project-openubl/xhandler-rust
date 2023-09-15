use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;

pub trait TipoComprobanteAfectadoGetter {
    fn get_tipo_comprobante_afectado(&self) -> &Option<&'static str>;
}

pub trait TipoComprobanteAfectadoSetter {
    fn set_tipo_comprobante_afectado(&mut self, val: &'static str);
}

impl TipoComprobanteAfectadoGetter for CreditNote {
    fn get_tipo_comprobante_afectado(&self) -> &Option<&'static str> {
        &self.comprobante_afectado_tipo
    }
}

impl TipoComprobanteAfectadoGetter for DebitNote {
    fn get_tipo_comprobante_afectado(&self) -> &Option<&'static str> {
        &self.comprobante_afectado_tipo
    }
}

impl TipoComprobanteAfectadoSetter for CreditNote {
    fn set_tipo_comprobante_afectado(&mut self, val: &'static str) {
        self.comprobante_afectado_tipo = Some(val);
    }
}

impl TipoComprobanteAfectadoSetter for DebitNote {
    fn set_tipo_comprobante_afectado(&mut self, val: &'static str) {
        self.comprobante_afectado_tipo = Some(val);
    }
}
