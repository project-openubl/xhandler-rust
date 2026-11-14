use anyhow::Result;

use crate::xbuilder::enricher::bounds::invoice::detraccion::{
    DetraccionMontoGetter, DetraccionMontoSetter, DetraccionPorcentajeGetter,
    InvoiceDetraccionGetter,
};
use crate::xbuilder::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::xbuilder::models::common::TotalImporteInvoice;

pub trait InvoiceDetraccionSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> InvoiceDetraccionSummaryRule for T
where
    T: InvoiceDetraccionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.total_importe().clone(), self.detraccion_mut()) {
            (Some(total_importe), Some(detraccion)) => {
                let results = [
                    DetraccionMontoSummaryRule::summary(detraccion, &total_importe)
                        .unwrap_or_default(),
                ];
                Ok(results.contains(&true))
            }
            _ => Ok(false),
        }
    }
}

//

pub trait DetraccionMontoSummaryRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool>;
}

impl<T> DetraccionMontoSummaryRule for T
where
    T: DetraccionMontoGetter + DetraccionMontoSetter + DetraccionPorcentajeGetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool> {
        match &self.monto() {
            Some(_) => Ok(false),
            None => {
                let monto = total_importe.importe_con_impuestos * self.porcentaje();
                self.set_monto(monto);
                Ok(true)
            }
        }
    }
}
