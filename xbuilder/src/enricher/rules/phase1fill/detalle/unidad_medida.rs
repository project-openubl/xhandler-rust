use anyhow::Result;

use crate::enricher::bounds::detalle::unidad_medida::{
    DetalleUnidadMedidaGetter, DetalleUnidadMedidaSetter,
};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleUnidadMedidaFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleUnidadMedidaFillRule for T
where
    T: DetalleUnidadMedidaGetter + DetalleUnidadMedidaSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match &self.get_unidad_medida() {
            Some(..) => Ok(false),
            None => {
                self.set_unidad_medida("NIU");
                Ok(true)
            }
        }
    }
}
