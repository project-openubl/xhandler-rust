use crate::xbuilder::models::perception_retention::Retention;

pub trait RetentionDocumentoIdGetter {
    fn documento_id(&self) -> &Option<String>;
}

pub trait RetentionDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait RetentionSerieGetter {
    fn serie(&self) -> &str;
}

pub trait RetentionNumeroGetter {
    fn numero(&self) -> u32;
}

impl RetentionDocumentoIdGetter for Retention {
    fn documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl RetentionDocumentoIdSetter for Retention {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl RetentionSerieGetter for Retention {
    fn serie(&self) -> &str {
        &self.serie
    }
}

impl RetentionNumeroGetter for Retention {
    fn numero(&self) -> u32 {
        self.numero
    }
}
