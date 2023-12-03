use rust_decimal::Decimal;

use crate::models::general::{FormaDePago, TipoFormaDePago};
use crate::models::traits::invoice::formadepago::{
    InvoiceFormaDePagoGetter, InvoiceFormaDePagoSetter,
};

pub trait InvoiceFormaDePagoEnrichRule {
    fn fill(&mut self) -> bool;
}

pub trait InvoiceFormaDePagoTotalRule {
    fn fill(&mut self) -> bool;
}

impl<T> InvoiceFormaDePagoEnrichRule for T
where
    T: InvoiceFormaDePagoGetter + InvoiceFormaDePagoSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_formadepago() {
            Some(..) => false,
            None => {
                self.set_formadepago(FormaDePago {
                    tipo: TipoFormaDePago::Contado,
                    cuotas: vec![],
                    total: None,
                });
                true
            }
        }
    }
}

impl<T> InvoiceFormaDePagoTotalRule for T
where
    T: InvoiceFormaDePagoGetter + InvoiceFormaDePagoSetter,
{
    fn fill(&mut self) -> bool {
        if let Some(forma_de_pago) = self.get_formadepago() {
            if forma_de_pago.total.is_none() {
                let total = forma_de_pago
                    .cuotas
                    .iter()
                    .map(|e| e.importe)
                    .fold(Decimal::ZERO, |accumulator, current| accumulator + current);

                self.set_formadepago(FormaDePago {
                    total: Some(total),
                    ..forma_de_pago.clone()
                });

                return true;
            };
        };

        false
    }
}
