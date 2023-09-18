use crate::catalogs::{Catalog, Catalog53};
use crate::models::traits::invoice::descuentos::{
    DescuentoGetter, DescuentoSetter, DescuentosGetter,
};

pub trait DescuentosRule {
    fn enrich_descuentos(&mut self) -> bool;
}

impl<T> DescuentosRule for T
where
    T: DescuentosGetter,
{
    fn enrich_descuentos(&mut self) -> bool {
        self.get_descuentos()
            .iter_mut()
            .map(|descuento| {
                let results = vec![
                    descuento.enrich_factor(),
                    descuento.enrich_monto_base(),
                    descuento.enrich_tipo(),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}

//

pub trait DescuentoRule {
    fn enrich_factor(&mut self) -> bool;
    fn enrich_monto_base(&mut self) -> bool;
    fn enrich_tipo(&mut self) -> bool;
}

impl<T> DescuentoRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn enrich_factor(&mut self) -> bool {
        match self.get_factor() {
            Some(..) => false,
            None => {
                self.set_factor(1f64);
                true
            }
        }
    }

    fn enrich_monto_base(&mut self) -> bool {
        match self.get_monto_base() {
            Some(..) => false,
            None => {
                self.set_monto_base(self.get_monto());
                true
            }
        }
    }

    fn enrich_tipo(&mut self) -> bool {
        match self.get_tipo() {
            Some(..) => false,
            None => {
                self.set_tipo(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap.code());
                true
            }
        }
    }
}
