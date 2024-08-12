use anyhow::Result;

use crate::catalogs::{Catalog, Catalog9};
use crate::enricher::bounds::note::creditnote::tipo_nota::{
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
            Some(..) => Ok(false),
            None => {
                self.set_tipo_nota(Catalog9::AnulacionDeLaOperacion.code());
                Ok(true)
            }
        }
    }
}
