use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;

pub trait NoteComprobanteAfectadoTipoGetter {
    fn comprobante_afectado_tipo(&self) -> &Option<String>;
}

pub trait NoteComprobanteAfectadoTipoSetter {
    fn set_comprobante_afectado_tipo(&mut self, val: &str);
}

impl NoteComprobanteAfectadoTipoGetter for CreditNote {
    fn comprobante_afectado_tipo(&self) -> &Option<String> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteComprobanteAfectadoTipoGetter for DebitNote {
    fn comprobante_afectado_tipo(&self) -> &Option<String> {
        &self.comprobante_afectado_tipo
    }
}

impl NoteComprobanteAfectadoTipoSetter for CreditNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &str) {
        self.comprobante_afectado_tipo = Some(val.to_string());
    }
}

impl NoteComprobanteAfectadoTipoSetter for DebitNote {
    fn set_comprobante_afectado_tipo(&mut self, val: &str) {
        self.comprobante_afectado_tipo = Some(val.to_string());
    }
}
