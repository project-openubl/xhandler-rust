use rust_decimal::Decimal;

use crate::models::despatch_advice::{DespatchAdvice, Remitente};

pub trait DespatchAdviceTipoComprobanteGetter {
    fn get_tipo_comprobante(&self) -> &Option<&'static str>;
}

pub trait DespatchAdviceTipoComprobanteSetter {
    fn set_tipo_comprobante(&mut self, val: &'static str);
}

pub trait DespatchAdviceSerieNumeroGetter {
    fn get_serie_numero(&self) -> &'static str;
}

pub trait DespatchAdviceRemitenteGetter {
    fn get_remitente(&self) -> &Remitente;
}

pub trait DespatchAdvicePesoTotalGetter {
    fn get_peso_total(&self) -> Decimal;
}

pub trait DespatchAdvicePesoTotalFormattedGetter {
    fn get_peso_total_formatted(&self) -> &Option<String>;
}

pub trait DespatchAdvicePesoTotalFormattedSetter {
    fn set_peso_total_formatted(&mut self, val: String);
}

impl DespatchAdviceTipoComprobanteGetter for DespatchAdvice {
    fn get_tipo_comprobante(&self) -> &Option<&'static str> {
        &self.tipo_comprobante
    }
}

impl DespatchAdviceTipoComprobanteSetter for DespatchAdvice {
    fn set_tipo_comprobante(&mut self, val: &'static str) {
        self.tipo_comprobante = Some(val);
    }
}

impl DespatchAdviceSerieNumeroGetter for DespatchAdvice {
    fn get_serie_numero(&self) -> &'static str {
        self.serie_numero
    }
}

impl DespatchAdviceRemitenteGetter for DespatchAdvice {
    fn get_remitente(&self) -> &Remitente {
        &self.remitente
    }
}

impl DespatchAdvicePesoTotalGetter for DespatchAdvice {
    fn get_peso_total(&self) -> Decimal {
        self.envio.peso_total
    }
}

impl DespatchAdvicePesoTotalFormattedGetter for DespatchAdvice {
    fn get_peso_total_formatted(&self) -> &Option<String> {
        &self.envio.peso_total_formatted
    }
}

impl DespatchAdvicePesoTotalFormattedSetter for DespatchAdvice {
    fn set_peso_total_formatted(&mut self, val: String) {
        self.envio.peso_total_formatted = Some(val);
    }
}
