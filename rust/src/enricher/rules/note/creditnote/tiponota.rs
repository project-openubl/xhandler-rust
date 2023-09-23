use crate::catalogs::{Catalog, Catalog9};
use crate::models::traits::note::creditnote::tiponota::{
    TipoNotaCreditoGetter, TipoNotaCreditoSetter,
};

pub trait CreditNoteTipoRule {
    fn enrich(&mut self) -> bool;
}

impl<T> CreditNoteTipoRule for T
where
    T: TipoNotaCreditoGetter + TipoNotaCreditoSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_tipo_nota_credito() {
            Some(..) => false,
            None => {
                self.set_tipo_nota_credito(Catalog9::AnulacionDeLaOperacion.code());
                true
            }
        }
    }
}
