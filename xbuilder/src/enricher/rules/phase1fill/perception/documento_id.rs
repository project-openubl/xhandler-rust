use anyhow::Result;

use crate::enricher::bounds::perception::{
    PerceptionDocumentoIdGetter, PerceptionDocumentoIdSetter, PerceptionNumeroGetter,
    PerceptionSerieGetter,
};

pub trait PerceptionDocumentoIdFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> PerceptionDocumentoIdFillRule for T
where
    T: PerceptionDocumentoIdGetter
        + PerceptionDocumentoIdSetter
        + PerceptionSerieGetter
        + PerceptionNumeroGetter,
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
