use crate::catalogs::{Catalog, Catalog10};
use crate::enricher::bounds::note::debitnote::tipo_nota::{
    DebitNoteTipoGetter, DebitNoteTipoSetter,
};

pub trait DebitNoteTipoFillRule {
    fn fill(&mut self) -> bool;
}

impl<T> DebitNoteTipoFillRule for T
where
    T: DebitNoteTipoGetter + DebitNoteTipoSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_tipo_nota() {
            Some(..) => false,
            None => {
                self.set_tipo_nota(Catalog10::AumentoEnElValor.code());
                true
            }
        }
    }
}
