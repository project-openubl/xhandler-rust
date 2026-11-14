use crate::xbuilder::models::despatch_advice::{DespatchAdvice, Remitente};

pub trait DespatchAdviceTipoComprobanteGetter {
    fn tipo_comprobante(&self) -> &Option<String>;
}

pub trait DespatchAdviceTipoComprobanteSetter {
    fn set_tipo_comprobante(&mut self, val: &str);
}

pub trait DespatchAdviceRemitenteGetter {
    fn remitente(&self) -> &Remitente;
}

impl DespatchAdviceTipoComprobanteGetter for DespatchAdvice {
    fn tipo_comprobante(&self) -> &Option<String> {
        &self.tipo_comprobante
    }
}

impl DespatchAdviceTipoComprobanteSetter for DespatchAdvice {
    fn set_tipo_comprobante(&mut self, val: &str) {
        self.tipo_comprobante = Some(val.to_string());
    }
}

impl DespatchAdviceRemitenteGetter for DespatchAdvice {
    fn remitente(&self) -> &Remitente {
        &self.remitente
    }
}
