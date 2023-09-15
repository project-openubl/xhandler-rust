use crate::catalogs::{Catalog, Catalog9};
use crate::models::traits::note::creditnote::tiponota::{
    TipoNotaCreditoGetter, TipoNotaCreditoSetter,
};

pub trait TipoNotaCreditoRule {
    fn enrich_tipo_nota_credito(&mut self) -> bool;
}

impl<T> TipoNotaCreditoRule for T
where
    T: TipoNotaCreditoGetter + TipoNotaCreditoSetter,
{
    fn enrich_tipo_nota_credito(&mut self) -> bool {
        match &self.get_tipo_nota_credito() {
            Some(..) => false,
            None => {
                self.set_tipo_nota_credito(Catalog9::AnulacionDeLaOperacion.code());
                true
            }
        }
    }
}
