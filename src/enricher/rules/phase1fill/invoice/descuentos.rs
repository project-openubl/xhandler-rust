use rust_decimal::Decimal;

use crate::catalogs::{Catalog, Catalog53};
use crate::enricher::bounds::invoice::descuentos::{
    DescuentoGetter, DescuentoSetter, InvoiceDescuentosGetter,
};

pub trait InvoiceDescuentosEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> InvoiceDescuentosEnrichRule for T
where
    T: InvoiceDescuentosGetter,
{
    fn fill(&mut self) -> bool {
        self.get_descuentos()
            .iter_mut()
            .map(|descuento| {
                let results = [
                    DescuentoFactorRule::fill(descuento),
                    DescuentoMontoBaseRule::fill(descuento),
                    DescuentoTipoRule::fill(descuento),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}

//

pub trait DescuentoFactorRule {
    fn fill(&mut self) -> bool;
}

pub trait DescuentoMontoBaseRule {
    fn fill(&mut self) -> bool;
}

pub trait DescuentoTipoRule {
    fn fill(&mut self) -> bool;
}

impl<T> DescuentoFactorRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> bool {
        match self.get_factor() {
            Some(..) => false,
            None => {
                self.set_factor(Decimal::ONE);
                true
            }
        }
    }
}

impl<T> DescuentoMontoBaseRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> bool {
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
    fn fill(&mut self) -> bool {
        match self.get_tipo() {
            Some(..) => false,
            None => {
                self.set_tipo(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap.code());
                true
            }
        }
    }
}
