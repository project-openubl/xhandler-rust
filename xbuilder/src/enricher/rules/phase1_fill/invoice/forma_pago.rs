use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::invoice::forma_pago::{
    InvoiceFormaPagoGetter, InvoiceFormaPagoSetter,
};
use crate::models::common::{FormaPago, TipoFormaPago};

pub trait InvoiceFormaPagoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceFormaPagoTotalFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceFormaPagoTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceFormaPagoFillRule for T
where
    T: InvoiceFormaPagoGetter + InvoiceFormaPagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.forma_pago() {
            Some(_) => Ok(false),
            None => {
                self.set_forma_pago(FormaPago {
                    tipo: Some(TipoFormaPago::Contado),
                    cuotas: vec![],
                    total: None,
                });
                Ok(true)
            }
        }
    }
}

impl<T> InvoiceFormaPagoTipoFillRule for T
where
    T: InvoiceFormaPagoGetter + InvoiceFormaPagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        if let Some(forma_pago) = self.forma_pago() {
            if forma_pago.tipo.is_none() {
                let tipo = if forma_pago.cuotas.is_empty() {
                    TipoFormaPago::Contado
                } else {
                    TipoFormaPago::Credito
                };
                self.set_forma_pago(FormaPago {
                    tipo: Some(tipo),
                    ..forma_pago.clone()
                });

                return Ok(true);
            };
        };

        Ok(false)
    }
}

impl<T> InvoiceFormaPagoTotalFillRule for T
where
    T: InvoiceFormaPagoGetter + InvoiceFormaPagoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        if let Some(forma_pago) = self.forma_pago() {
            if forma_pago.total.is_none() {
                let total = forma_pago
                    .cuotas
                    .iter()
                    .map(|e| e.importe)
                    .fold(Decimal::ZERO, |accumulator, current| accumulator + current);

                self.set_forma_pago(FormaPago {
                    total: Some(total),
                    ..forma_pago.clone()
                });

                return Ok(true);
            };
        };

        Ok(false)
    }
}
