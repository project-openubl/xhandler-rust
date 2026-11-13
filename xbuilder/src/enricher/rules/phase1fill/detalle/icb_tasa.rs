use anyhow::Result;

use crate::enricher::bounds::detalle::icb_tasa::{DetalleIcbTasaGetter, DetalleIcbTasaSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIcbTasaFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleIcbTasaFillRule for T
where
    T: DetalleIcbTasaGetter + DetalleIcbTasaSetter,
{
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool> {
        match &self.get_icb_tasa() {
            Some(..) => Ok(false),
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                Ok(true)
            }
        }
    }
}
