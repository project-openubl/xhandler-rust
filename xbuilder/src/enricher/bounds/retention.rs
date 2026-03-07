use crate::models::perception_retention::Retention;

pub trait RetentionDocumentoIdGetter {
    fn get_documento_id(&self) -> &Option<String>;
}

pub trait RetentionDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait RetentionSerieGetter {
    fn get_serie(&self) -> &str;
}

pub trait RetentionNumeroGetter {
    fn get_numero(&self) -> u32;
}

pub trait RetentionMonedaGetter {
    fn get_moneda(&self) -> &Option<String>;
}

pub trait RetentionMonedaSetter {
    fn set_moneda(&mut self, val: &str);
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
    fn get_serie(&self) -> &str {
        &self.serie
    }
}

impl RetentionNumeroGetter for Retention {
    fn get_numero(&self) -> u32 {
        self.numero
    }
}

impl RetentionMonedaGetter for Retention {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl RetentionMonedaSetter for Retention {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}
