use crate::enricher::bounds::detalle::unidad_medida::{
    DetalleUnidadMedidaGetter, DetalleUnidadMedidaSetter,
};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleUnidadMedidaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleUnidadMedidaEnrichRule for T
where
    T: DetalleUnidadMedidaGetter + DetalleUnidadMedidaSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_unidad_medida() {
            Some(..) => false,
            None => {
                self.set_unidad_medida("NIU");
                true
            }
        }
    }
}
