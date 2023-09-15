use crate::models::common::{FormaDePago, TipoFormaDePago};
use crate::models::traits::invoice::formadepago::{FormaDePagoGetter, FormaDePagoSetter};

pub trait FormaDePagoRule {
    fn enrich_formadepago(&mut self) -> bool;
}

pub trait FormaDePagoTotalRule {
    fn enrich_formadepago_total(&mut self) -> bool;
}

impl<T> FormaDePagoRule for T
where
    T: FormaDePagoGetter + FormaDePagoSetter,
{
    fn enrich_formadepago(&mut self) -> bool {
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

impl<T> FormaDePagoTotalRule for T
where
    T: FormaDePagoGetter + FormaDePagoSetter,
{
    fn enrich_formadepago_total(&mut self) -> bool {
        if let Some(forma_de_pago) = self.get_formadepago() {
            if forma_de_pago.total.is_none() {
                let total = forma_de_pago
                    .cuotas
                    .iter()
                    .map(|e| e.importe)
                    .fold(0f64, |accumulator, current| accumulator + current);

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
