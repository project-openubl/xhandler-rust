use anyhow::Result;

use crate::enricher::bounds::moneda::{MonedaGetter, MonedaSetter};

pub trait MonedaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> MonedaFillRule for T
where
    T: MonedaGetter + MonedaSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_moneda() {
            Some(..) => Ok(false),
            None => {
                self.set_moneda("PEN");
                Ok(true)
            }
        }
    }
}
