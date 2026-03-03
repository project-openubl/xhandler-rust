use anyhow::Result;

use crate::enricher::bounds::retention::{
    RetentionDocumentoIdGetter, RetentionDocumentoIdSetter, RetentionNumeroGetter,
    RetentionSerieGetter,
};

pub trait RetentionDocumentoIdFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> RetentionDocumentoIdFillRule for T
where
    T: RetentionDocumentoIdGetter
        + RetentionDocumentoIdSetter
        + RetentionSerieGetter
        + RetentionNumeroGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_documento_id() {
            Some(_) => Ok(false),
            None => {
                let id = format!("{}-{}", self.get_serie(), self.get_numero());
                self.set_documento_id(id);
                Ok(true)
            }
        }
    }
}
