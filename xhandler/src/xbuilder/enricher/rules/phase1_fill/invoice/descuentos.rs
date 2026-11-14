use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{code_for_name, Catalog, Catalog53};
use crate::xbuilder::enricher::bounds::invoice::descuentos::{
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
        let result = self.descuentos_mut().iter_mut().any(|descuento| {
            let results = [
                DescuentoFactorFillRule::fill(descuento).unwrap_or_default(),
                DescuentoMontoBaseFillRule::fill(descuento).unwrap_or_default(),
                DescuentoTipoFillRule::fill(descuento).unwrap_or_default(),
            ];
            results.contains(&true)
        });
        Ok(result)
    }
}

//

pub trait DescuentoFactorFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait DescuentoMontoBaseFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait DescuentoTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DescuentoFactorFillRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.factor() {
            Some(..) => Ok(false),
            None => {
                self.set_factor(Decimal::ONE);
                Ok(true)
            }
        }
    }
}

impl<T> DescuentoMontoBaseFillRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.monto_base() {
            Some(..) => Ok(false),
            None => {
                self.set_monto_base(self.monto());
                Ok(true)
            }
        }
    }
}

impl<T> DescuentoTipoFillRule for T
where
    T: DescuentoGetter + DescuentoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.tipo() {
            Some(value) => match code_for_name::<Catalog53>(value) {
                Some(code) => {
                    self.set_tipo(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_tipo(Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap.code());
                Ok(true)
            }
        }
    }
}
