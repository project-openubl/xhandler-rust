use anyhow::Result;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog, Catalog53};
use crate::enricher::bounds::invoice::descuentos::{
    DescuentoGetter, DescuentoSetter, InvoiceDescuentosGetter,
};

pub trait InvoiceDescuentosFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceDescuentosFillRule for T
where
    T: InvoiceDescuentosGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let result = self
            .get_descuentos()
            .iter_mut()
            .map(|descuento| {
                let results = [
                    DescuentoFactorRule::fill(descuento).unwrap_or_default(),
                    DescuentoMontoBaseRule::fill(descuento).unwrap_or_default(),
                    DescuentoTipoRule::fill(descuento).unwrap_or_default(),
                ];
                results.contains(&true)
            })
            .any(|changed| changed);
        Ok(result)
    }
}

//

pub trait DescuentoFactorRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait DescuentoMontoBaseRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait DescuentoTipoRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DescuentoFactorRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.get_factor() {
            Some(..) => Ok(false),
            None => {
                self.set_factor(Decimal::ONE);
                Ok(true)
            }
        }
    }
}

impl<T> DescuentoMontoBaseRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.get_monto_base() {
            Some(..) => Ok(false),
            None => {
                self.set_monto_base(self.get_monto());
                Ok(true)
            }
        }
    }
}

impl<T> DescuentoTipoRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.get_tipo() {
            Some(..) => Ok(false),
            None => {
                self.set_tipo(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap.code());
                Ok(true)
            }
        }
    }
}
