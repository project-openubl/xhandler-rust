use crate::models::traits::invoice::detraccion::InvoiceDetraccionGetter;

pub trait RetencionProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> RetencionProcessRule for T
where
    T: InvoiceDetraccionGetter,
{
    fn process(&mut self) -> bool {
        if let Some(detraccion) = self.get_detraccion() {
            let _ = detraccion;
            let results = vec![];
            results.contains(&true)
        } else {
            false
        }
    }
}
