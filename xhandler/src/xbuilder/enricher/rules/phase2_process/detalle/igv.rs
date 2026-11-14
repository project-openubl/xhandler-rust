use anyhow::Result;

use crate::xbuilder::enricher::bounds::detalle::igv::{DetalleIgvGetter, DetalleIgvSetter};
use crate::xbuilder::enricher::bounds::detalle::igv_base_imponible::DetalleIgvBaseImponibleGetter;
use crate::xbuilder::enricher::bounds::detalle::igv_tasa::DetalleIgvTasaGetter;

pub trait DetalleIgvProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIgvProcessRule for T
where
    T: DetalleIgvGetter + DetalleIgvSetter + DetalleIgvBaseImponibleGetter + DetalleIgvTasaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (&self.igv(), &self.igv_base_imponible(), &self.igv_tasa()) {
            (None, Some(igv_base_imponible), Some(igv_tasa)) => {
                let igv = igv_base_imponible * *igv_tasa;
                self.set_igv(igv);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
