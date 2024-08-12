use anyhow::Result;

use crate::enricher::bounds::invoice::detraccion::{
    InvoiceDetraccionGetter, InvoiceDetraccionMontoGetter, InvoiceDetraccionMontoSetter,
    InvoiceDetraccionPorcentajeGetter,
};
use crate::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::models::common::TotalImporteInvoice;

pub trait DetraccionSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> DetraccionSummaryRule for T
where
    T: InvoiceDetraccionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.get_total_importe().clone(), self.get_detraccion()) {
            (Some(total_importe), Some(detraccion)) => {
                let results = [
                    DetraccionMontoRule::summary(detraccion, &total_importe).unwrap_or_default()
                ];
                Ok(results.contains(&true))
            }
            _ => Ok(false),
        }
    }
}

//

pub trait DetraccionMontoRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool>;
}

impl<T> DetraccionMontoRule for T
where
    T: InvoiceDetraccionMontoGetter
        + InvoiceDetraccionMontoSetter
        + InvoiceDetraccionPorcentajeGetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool> {
        match &self.get_monto() {
            Some(_) => Ok(false),
            None => {
                let monto = total_importe.importe_con_impuestos * self.get_porcentaje();
                self.set_monto(monto);
                Ok(true)
            }
        }
    }
}
