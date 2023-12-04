use crate::catalogs::{Catalog, Catalog9};
use crate::models::traits::note::creditnote::tiponota::{
    CreditNoteTipoGetter, CreditNoteTipoSetter,
};

pub trait CreditNoteTipoEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> CreditNoteTipoEnrichRule for T
where
    T: CreditNoteTipoGetter + CreditNoteTipoSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_tipo_nota_credito() {
            Some(..) => false,
            None => {
                self.set_tipo_nota_credito(Catalog9::AnulacionDeLaOperacion.code());
                true
            }
        }
    }
}
