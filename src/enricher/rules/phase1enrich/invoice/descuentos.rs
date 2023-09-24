use crate::catalogs::{Catalog, Catalog53};
use crate::models::traits::invoice::descuentos::{
    DescuentoGetter, DescuentoSetter, InvoiceDescuentosGetter,
};

pub trait InvoiceDescuentosEnrichRule {
    fn enrich(&mut self) -> bool;
}

impl<T> InvoiceDescuentosEnrichRule for T
where
    T: InvoiceDescuentosGetter,
{
    fn enrich(&mut self) -> bool {
        self.get_descuentos()
            .iter_mut()
            .map(|descuento| {
                let results = vec![
                    DescuentoFactorRule::enrich(descuento),
                    DescuentoMontoBaseRule::enrich(descuento),
                    DescuentoTipoRule::enrich(descuento),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}

//

pub trait DescuentoFactorRule {
    fn enrich(&mut self) -> bool;
}

pub trait DescuentoMontoBaseRule {
    fn enrich(&mut self) -> bool;
}

pub trait DescuentoTipoRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DescuentoFactorRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn enrich(&mut self) -> bool {
        match self.get_factor() {
            Some(..) => false,
            None => {
                self.set_factor(1f64);
                true
            }
        }
    }
}

impl<T> DescuentoMontoBaseRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn enrich(&mut self) -> bool {
        match self.get_monto_base() {
            Some(..) => false,
            None => {
                self.set_monto_base(self.get_monto());
                true
            }
        }
    }
}

impl<T> DescuentoTipoRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn enrich(&mut self) -> bool {
        match self.get_tipo() {
            Some(..) => false,
            None => {
                self.set_tipo(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap.code());
                true
            }
        }
    }
}
