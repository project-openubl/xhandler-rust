use crate::models::traits::invoice::percepcion::InvoicePercepcionGetter;

pub trait PercepcionProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> PercepcionProcessRule for T
where
    T: InvoicePercepcionGetter,
{
    fn process(&mut self) -> bool {
        if let Some(percepcion) = self.get_percepcion() {
            let _ = percepcion;
            let results = vec![];
            results.contains(&true)
        } else {
            false
        }
    }
}
