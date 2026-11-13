use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::invoice::forma_de_pago::{
    InvoiceFormaDePagoGetter, InvoiceFormaDePagoSetter,
};
use crate::models::common::{FormaDePago, TipoFormaDePago};

pub trait InvoiceFormaDePagoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceFormaDePagoTotalRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceFormaDePagoTipoRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceFormaDePagoFillRule for T
where
    T: InvoiceFormaDePagoGetter + InvoiceFormaDePagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_forma_de_pago() {
            Some(_) => Ok(false),
            None => {
                self.set_forma_de_pago(FormaDePago {
                    tipo: Some(TipoFormaDePago::Contado),
                    cuotas: vec![],
                    total: None,
                });
                Ok(true)
            }
        }
    }
}

impl<T> InvoiceFormaDePagoTipoRule for T
where
    T: InvoiceFormaDePagoGetter + InvoiceFormaDePagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        if let Some(forma_de_pago) = self.get_forma_de_pago() {
            if forma_de_pago.tipo.is_none() {
                let tipo = if forma_de_pago.cuotas.is_empty() {
                    TipoFormaDePago::Contado
                } else {
                    TipoFormaDePago::Credito
                };
                self.set_forma_de_pago(FormaDePago {
                    tipo: Some(tipo),
                    ..forma_de_pago.clone()
                });

                return Ok(true);
            };
        };

        Ok(false)
    }
}

impl<T> InvoiceFormaDePagoTotalRule for T
where
    T: InvoiceFormaDePagoGetter + InvoiceFormaDePagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        if let Some(forma_de_pago) = self.get_forma_de_pago() {
            if forma_de_pago.total.is_none() {
                let total = forma_de_pago
                    .cuotas
                    .iter()
                    .map(|e| e.importe)
                    .fold(Decimal::ZERO, |accumulator, current| accumulator + current);

                self.set_forma_de_pago(FormaDePago {
                    total: Some(total),
                    ..forma_de_pago.clone()
                });

                return Ok(true);
            };
        };

        Ok(false)
    }
}
