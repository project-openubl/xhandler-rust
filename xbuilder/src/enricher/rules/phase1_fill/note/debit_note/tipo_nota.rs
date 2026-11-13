use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog, Catalog10};
use crate::enricher::bounds::note::debit_note::tipo_nota::{
    DebitNoteTipoNotaGetter, DebitNoteTipoNotaSetter,
};

pub trait DebitNoteTipoNotaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DebitNoteTipoNotaFillRule for T
where
    T: DebitNoteTipoNotaGetter + DebitNoteTipoNotaSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.tipo_nota() {
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
