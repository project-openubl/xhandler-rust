use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog, Catalog9};
use crate::enricher::bounds::note::credit_note::tipo_nota::{
    CreditNoteTipoNotaGetter, CreditNoteTipoNotaSetter,
};

pub trait CreditNoteTipoNotaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> CreditNoteTipoNotaFillRule for T
where
    T: CreditNoteTipoNotaGetter + CreditNoteTipoNotaSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.tipo_nota() {
            Some(value) => match code_for_name::<Catalog9>(value) {
                Some(code) => {
                    self.set_tipo_nota(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_tipo_nota(Catalog9::AnulacionDeLaOperacion.code());
                Ok(true)
            }
        }
    }
}
