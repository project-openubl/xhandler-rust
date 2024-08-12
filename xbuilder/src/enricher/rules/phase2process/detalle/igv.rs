use anyhow::Result;

use crate::enricher::bounds::detalle::igv::{DetalleIgvGetter, DetalleIgvSetter};
use crate::enricher::bounds::detalle::igv_base_imponible::DetalleIgvBaseImponibleGetter;
use crate::enricher::bounds::detalle::igv_tasa::DetalleIgvTasaGetter;

pub trait DetalleIGVProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIGVProcessRule for T
where
    T: DetalleIgvGetter + DetalleIgvSetter + DetalleIgvBaseImponibleGetter + DetalleIgvTasaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.get_igv(),
            &self.get_igv_base_imponible(),
            &self.get_igv_tasa(),
        ) {
            (None, Some(igv_base_imponible), Some(igv_tasa)) => {
                let igv = igv_base_imponible * *igv_tasa;
                self.set_igv(igv);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
