use anyhow::Result;

use crate::enricher::bounds::perception::{PerceptionMonedaGetter, PerceptionMonedaSetter};

pub trait PerceptionMonedaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> PerceptionMonedaFillRule for T
where
    T: PerceptionMonedaGetter + PerceptionMonedaSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_moneda() {
            Some(_) => Ok(false),
            None => {
                self.set_moneda("PEN");
                Ok(true)
            }
        }
    }
}
