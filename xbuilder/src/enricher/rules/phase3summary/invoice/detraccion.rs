use crate::enricher::bounds::invoice::detraccion::{
    InvoiceDetraccionGetter, InvoiceDetraccionMontoGetter, InvoiceDetraccionMontoSetter,
    InvoiceDetraccionPorcentajeGetter,
};
use crate::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::prelude::TotalImporteInvoice;

pub trait DetraccionSummaryRule {
    fn summary(&mut self) -> bool;
}

impl<T> DetraccionSummaryRule for T
where
    T: InvoiceDetraccionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> bool {
        match (self.get_total_importe().clone(), self.get_detraccion()) {
            (Some(total_importe), Some(detraccion)) => {
                let results = [DetraccionMontoRule::summary(detraccion, &total_importe)];
                results.contains(&true)
            }
            _ => false,
        }
    }
}

//

pub trait DetraccionMontoRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> bool;
}

impl<T> DetraccionMontoRule for T
where
    T: InvoiceDetraccionMontoGetter
        + InvoiceDetraccionMontoSetter
        + InvoiceDetraccionPorcentajeGetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> bool {
        match &self.get_monto() {
            Some(_) => false,
            None => {
                let monto = total_importe.importe_con_impuestos * self.get_porcentaje();
                self.set_monto(monto);
                true
            }
        }
    }
}
