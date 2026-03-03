use crate::models::perception_retention::Perception;

pub trait PerceptionDocumentoIdGetter {
    fn get_documento_id(&self) -> &Option<String>;
}

pub trait PerceptionDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait PerceptionSerieGetter {
    fn get_serie(&self) -> &'static str;
}

pub trait PerceptionNumeroGetter {
    fn get_numero(&self) -> u32;
}

pub trait PerceptionMonedaGetter {
    fn get_moneda(&self) -> &Option<&'static str>;
}

pub trait PerceptionMonedaSetter {
    fn set_moneda(&mut self, val: &'static str);
}

impl PerceptionDocumentoIdGetter for Perception {
    fn get_documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl PerceptionDocumentoIdSetter for Perception {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl PerceptionSerieGetter for Perception {
    fn get_serie(&self) -> &'static str {
        self.serie
    }
}

impl PerceptionNumeroGetter for Perception {
    fn get_numero(&self) -> u32 {
        self.numero
    }
}

impl PerceptionMonedaGetter for Perception {
    fn get_moneda(&self) -> &Option<&'static str> {
        &self.moneda
    }
}

impl PerceptionMonedaSetter for Perception {
    fn set_moneda(&mut self, val: &'static str) {
        self.moneda = Some(val);
    }
}
