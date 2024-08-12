use anyhow::Result;

use crate::enricher::bounds::igv::{IgvTasaGetter, IgvTasaSetter};
use crate::enricher::Defaults;

pub trait IgvTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> IgvTasaFillRule for T
where
    T: IgvTasaGetter + IgvTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        match &self.get_igv_tasa() {
            Some(..) => Ok(false),
            None => {
                self.set_igv_tasa(defaults.igv_tasa);
                Ok(true)
            }
        }
    }
}
