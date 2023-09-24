use crate::catalogs::{Catalog, Catalog10};
use crate::models::traits::note::debitnote::tiponota::{
    TipoNotaDebitoGetter, TipoNotaDebitoSetter,
};

pub trait DebitNoteTipoRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DebitNoteTipoRule for T
where
    T: TipoNotaDebitoGetter + TipoNotaDebitoSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_tipo_nota_debito() {
            Some(..) => false,
            None => {
                self.set_tipo_nota_debito(Catalog10::AumentoEnElValor.code());
                true
            }
        }
    }
}
