use anyhow::Result;

use crate::enricher::bounds::retention::{RetentionMonedaGetter, RetentionMonedaSetter};

pub trait RetentionMonedaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> RetentionMonedaFillRule for T
where
    T: RetentionMonedaGetter + RetentionMonedaSetter,
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
