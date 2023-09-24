use crate::enricher::rules::phase1enrich::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::unidadmedida::{
    DetalleUnidadMedidaGetter, DetalleUnidadMedidaSetter,
};

pub trait DetalleUnidadMedidaEnrichRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleUnidadMedidaEnrichRule for T
where
    T: DetalleUnidadMedidaGetter + DetalleUnidadMedidaSetter,
{
    fn enrich(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_unidadmedida() {
            Some(..) => false,
            None => {
                self.set_unidadmedida("NIU");
                true
            }
        }
    }
}
