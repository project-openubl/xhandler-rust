use crate::enricher::rules::phase2process::detalle::icb::DetalleICBProcessRule;
use crate::models::traits::detalle::DetallesGetter;

pub trait DetallesProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallesProcessRule for T
where
    T: DetallesGetter,
{
    fn process(&mut self) -> bool {
        self.get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = vec![DetalleICBProcessRule::process(detalle)];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
