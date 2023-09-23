use crate::enricher::rules::detalle::icbtasa::DetalleICBTasaRule;
use crate::enricher::rules::detalle::unidadmedida::DetalleUnidadMedidaRule;
use crate::models::traits::detalle::DetallesGetter;

pub trait DetallesRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DetallesRule for T
where
    T: DetallesGetter,
{
    fn enrich(&mut self) -> bool {
        self.get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = vec![
                    DetalleICBTasaRule::enrich(detalle),
                    DetalleUnidadMedidaRule::enrich(detalle),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
