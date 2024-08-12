use anyhow::Result;
use Ok;

use crate::enricher::bounds::ivap::{IvapTasaGetter, IvapTasaSetter};
use crate::enricher::Defaults;

pub trait IvapTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> IvapTasaFillRule for T
where
    T: IvapTasaGetter + IvapTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        match &self.get_ivap_tasa() {
            Some(..) => Ok(false),
            None => {
                self.set_ivap_tasa(defaults.ivap_tasa);
                Ok(true)
            }
        }
    }
}
