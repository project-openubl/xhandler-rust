use crate::catalogs::{Catalog, Catalog10};
use crate::models::traits::note::debitnote::tiponota::{DebitNoteTipoGetter, DebitNoteTipoSetter};

pub trait DebitNoteTipoEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> DebitNoteTipoEnrichRule for T
where
    T: DebitNoteTipoGetter + DebitNoteTipoSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_tipo_nota_debito() {
            Some(..) => false,
            None => {
                self.set_tipo_nota_debito(Catalog10::AumentoEnElValor.code());
                true
            }
        }
    }
}
