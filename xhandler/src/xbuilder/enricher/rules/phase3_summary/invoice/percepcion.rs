use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::enricher::bounds::invoice::percepcion::{
    InvoicePercepcionGetter, PercepcionMontoBaseGetter, PercepcionMontoBaseSetter,
    PercepcionMontoGetter, PercepcionMontoSetter, PercepcionMontoTotalGetter,
    PercepcionMontoTotalSetter, PercepcionPorcentajeGetter, PercepcionPorcentajeSetter,
};
use crate::xbuilder::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::xbuilder::models::common::TotalImporteInvoice;

pub trait InvoicePercepcionSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> InvoicePercepcionSummaryRule for T
where
    T: InvoicePercepcionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.total_importe().clone(), self.percepcion_mut()) {
            (Some(total_importe), Some(percepcion)) => [
                PercepcionPorcentajeSummaryRule::summary(percepcion).unwrap_or_default(),
                PercepcionMontoBaseSummaryRule::summary(percepcion, &total_importe)
                    .unwrap_or_default(),
                PercepcionMontoSummaryRule::summary(percepcion).unwrap_or_default(),
                PercepcionMontoTotalSummaryRule::summary(percepcion).unwrap_or_default(),
            ]
            .contains(&true),
            _ => false,
        };
        Ok(false)
    }
}

//

pub trait PercepcionPorcentajeSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PercepcionPorcentajeSummaryRule for T
where
    T: PercepcionPorcentajeGetter + PercepcionPorcentajeSetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.porcentaje() {
            Some(_) => Ok(false),
            None => {
                self.set_porcentaje(Decimal::ONE);
                Ok(true)
            }
        }
    }
}

//

pub trait PercepcionMontoBaseSummaryRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool>;
}

impl<T> PercepcionMontoBaseSummaryRule for T
where
    T: PercepcionMontoBaseGetter + PercepcionMontoBaseSetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool> {
        match &self.monto_base() {
            Some(_) => Ok(false),
            None => {
                self.set_monto_base(total_importe.importe_sin_impuestos);
                Ok(true)
            }
        }
    }
}

//

pub trait PercepcionMontoSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PercepcionMontoSummaryRule for T
where
    T: PercepcionMontoGetter
        + PercepcionMontoSetter
        + PercepcionMontoBaseGetter
        + PercepcionPorcentajeGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.monto(), self.monto_base(), self.porcentaje()) {
            (None, Some(monto_base), Some(porcentaje)) => {
                self.set_monto(monto_base * porcentaje);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

//

pub trait PercepcionMontoTotalSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PercepcionMontoTotalSummaryRule for T
where
    T: PercepcionMontoTotalGetter
        + PercepcionMontoTotalSetter
        + PercepcionMontoBaseGetter
        + PercepcionMontoGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.monto_total(), self.monto_base(), self.monto()) {
            (None, Some(monto_total), Some(monto)) => {
                self.set_monto_total(monto_total + monto);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}
