use anyhow::Result;

use crate::enricher::bounds::fecha_emision::{FechaEmisionGetter, FechaEmisionSetter};
use crate::enricher::Defaults;

pub trait FechaEmisionFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> FechaEmisionFillRule for T
where
    T: FechaEmisionGetter + FechaEmisionSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        match &self.get_fecha_emision() {
            Some(..) => Ok(false),
            None => {
                self.set_fecha_emision(defaults.date);
                Ok(true)
            }
        }
    }
}
