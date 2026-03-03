use crate::models::perception_retention::Retention;

pub trait RetentionDocumentoIdGetter {
    fn get_documento_id(&self) -> &Option<String>;
}

pub trait RetentionDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait RetentionSerieGetter {
    fn get_serie(&self) -> &'static str;
}

pub trait RetentionNumeroGetter {
    fn get_numero(&self) -> u32;
}

pub trait RetentionMonedaGetter {
    fn get_moneda(&self) -> &Option<&'static str>;
}

pub trait RetentionMonedaSetter {
    fn set_moneda(&mut self, val: &'static str);
}

impl RetentionDocumentoIdGetter for Retention {
    fn get_documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl RetentionDocumentoIdSetter for Retention {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl RetentionSerieGetter for Retention {
    fn get_serie(&self) -> &'static str {
        self.serie
    }
}

impl RetentionNumeroGetter for Retention {
    fn get_numero(&self) -> u32 {
        self.numero
    }
}

impl RetentionMonedaGetter for Retention {
    fn get_moneda(&self) -> &Option<&'static str> {
        &self.moneda
    }
}

impl RetentionMonedaSetter for Retention {
    fn set_moneda(&mut self, val: &'static str) {
        self.moneda = Some(val);
    }
}
