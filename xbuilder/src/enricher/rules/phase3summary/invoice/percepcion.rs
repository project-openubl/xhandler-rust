use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::invoice::percepcion::{
    InvoicePercepcionGetter, InvoicePercepcionMontoBaseGetter, InvoicePercepcionMontoBaseSetter,
    InvoicePercepcionMontoGetter, InvoicePercepcionMontoSetter, InvoicePercepcionMontoTotalGetter,
    InvoicePercepcionMontoTotalSetter, InvoicePercepcionPorcentajeGetter,
    InvoicePercepcionPorcentajeSetter,
};
use crate::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::models::common::TotalImporteInvoice;

pub trait PercepcionSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PercepcionSummaryRule for T
where
    T: InvoicePercepcionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (self.get_total_importe().clone(), self.get_percepcion()) {
            (Some(total_importe), Some(percepcion)) => [
                PerceptionPorcentajeBaseRule::summary(percepcion).unwrap_or_default(),
                PerceptionMontoBaseRule::summary(percepcion, &total_importe).unwrap_or_default(),
                PerceptionMontoRule::summary(percepcion).unwrap_or_default(),
                PerceptionMontoTotalRule::summary(percepcion).unwrap_or_default(),
            ]
            .contains(&true),
            _ => false,
        };
        Ok(false)
    }
}

//

pub trait PerceptionPorcentajeBaseRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PerceptionPorcentajeBaseRule for T
where
    T: InvoicePercepcionPorcentajeGetter + InvoicePercepcionPorcentajeSetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.get_porcentaje() {
            Some(_) => Ok(false),
            None => {
                self.set_porcentaje(Decimal::ONE);
                Ok(true)
            }
        }
    }
}

//

pub trait PerceptionMontoBaseRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool>;
}

impl<T> PerceptionMontoBaseRule for T
where
    T: InvoicePercepcionMontoBaseGetter + InvoicePercepcionMontoBaseSetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> Result<bool> {
        match &self.get_montobase() {
            Some(_) => Ok(false),
            None => {
                self.set_montobase(total_importe.importe_sin_impuestos);
                Ok(true)
            }
        }
    }
}

//

pub trait PerceptionMontoRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PerceptionMontoRule for T
where
    T: InvoicePercepcionMontoGetter
        + InvoicePercepcionMontoSetter
        + InvoicePercepcionMontoBaseGetter
        + InvoicePercepcionPorcentajeGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (
            self.get_monto(),
            self.get_montobase(),
            self.get_porcentaje(),
        ) {
            (None, Some(monto_base), Some(porcentaje)) => {
                self.set_monto(monto_base * porcentaje);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

//

pub trait PerceptionMontoTotalRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> PerceptionMontoTotalRule for T
where
    T: InvoicePercepcionMontoTotalGetter
        + InvoicePercepcionMontoTotalSetter
        + InvoicePercepcionMontoBaseGetter
        + InvoicePercepcionMontoGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match (
            self.get_montototal(),
            self.get_montobase(),
            self.get_monto(),
        ) {
            (None, Some(monto_total), Some(monto)) => {
                self.set_montototal(monto_total + monto);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}
