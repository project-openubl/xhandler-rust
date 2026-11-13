use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog, Catalog9};
use crate::enricher::bounds::note::credit_note::tipo_nota::{
    CreditNoteTipoGetter, CreditNoteTipoSetter,
};

pub trait CreditNoteTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> CreditNoteTipoFillRule for T
where
    T: CreditNoteTipoGetter + CreditNoteTipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_tipo_nota() {
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
