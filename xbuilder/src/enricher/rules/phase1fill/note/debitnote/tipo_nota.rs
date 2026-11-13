use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog, Catalog10};
use crate::enricher::bounds::note::debitnote::tipo_nota::{
    DebitNoteTipoGetter, DebitNoteTipoSetter,
};

pub trait DebitNoteTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DebitNoteTipoFillRule for T
where
    T: DebitNoteTipoGetter + DebitNoteTipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_tipo_nota() {
            Some(value) => match code_for_name::<Catalog10>(value) {
                Some(code) => {
                    self.set_tipo_nota(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_tipo_nota(Catalog10::AumentoEnElValor.code());
                Ok(true)
            }
        }
    }
}
