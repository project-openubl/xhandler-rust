use crate::models::traits::detalle::unidadmedida::{UnidadMedidaGetter, UnidadMedidaSetter};

pub trait DetalleUnidadMedidaRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DetalleUnidadMedidaRule for T
where
    T: UnidadMedidaGetter + UnidadMedidaSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_unidadmedida() {
            Some(..) => false,
            None => {
                self.set_unidadmedida("NIU");
                true
            }
        }
    }
}
