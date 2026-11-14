use anyhow::Result;

use crate::xbuilder::enricher::bounds::icb_tasa::{IcbTasaGetter, IcbTasaSetter};
use crate::xbuilder::enricher::Defaults;

pub trait IcbTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> IcbTasaFillRule for T
where
    T: IcbTasaGetter + IcbTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        match &self.icb_tasa() {
            Some(..) => Ok(false),
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                Ok(true)
            }
        }
    }
}
