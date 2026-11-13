use crate::models::perception_retention::Perception;

pub trait PerceptionDocumentoIdGetter {
    fn documento_id(&self) -> &Option<String>;
}

pub trait PerceptionDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait PerceptionSerieGetter {
    fn serie(&self) -> &str;
}

pub trait PerceptionNumeroGetter {
    fn numero(&self) -> u32;
}

impl PerceptionDocumentoIdGetter for Perception {
    fn documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl PerceptionDocumentoIdSetter for Perception {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl PerceptionSerieGetter for Perception {
    fn serie(&self) -> &str {
        &self.serie
    }
}

impl PerceptionNumeroGetter for Perception {
    fn numero(&self) -> u32 {
        self.numero
    }
}
