use rust_decimal::Decimal;

use crate::enricher::bounds::invoice::percepcion::{
    InvoicePercepcionGetter, InvoicePercepcionMontoBaseGetter, InvoicePercepcionMontoBaseSetter,
    InvoicePercepcionMontoGetter, InvoicePercepcionMontoSetter, InvoicePercepcionMontoTotalGetter,
    InvoicePercepcionMontoTotalSetter, InvoicePercepcionPorcentajeGetter,
    InvoicePercepcionPorcentajeSetter,
};
use crate::enricher::bounds::invoice::total_importe::InvoiceTotalImporteGetter;
use crate::prelude::TotalImporteInvoice;

pub trait PercepcionSummaryRule {
    fn summary(&mut self) -> bool;
}

impl<T> PercepcionSummaryRule for T
where
    T: InvoicePercepcionGetter + InvoiceTotalImporteGetter,
{
    fn summary(&mut self) -> bool {
        match (self.get_total_importe().clone(), self.get_percepcion()) {
            (Some(total_importe), Some(percepcion)) => [
                PerceptionPorcentajeBaseRule::summary(percepcion),
                PerceptionMontoBaseRule::summary(percepcion, &total_importe),
                PerceptionMontoRule::summary(percepcion),
                PerceptionMontoTotalRule::summary(percepcion),
            ]
            .contains(&true),
            _ => false,
        };
        false
    }
}

//

pub trait PerceptionPorcentajeBaseRule {
    fn summary(&mut self) -> bool;
}

impl<T> PerceptionPorcentajeBaseRule for T
where
    T: InvoicePercepcionPorcentajeGetter + InvoicePercepcionPorcentajeSetter,
{
    fn summary(&mut self) -> bool {
        match &self.get_porcentaje() {
            Some(_) => false,
            None => {
                self.set_porcentaje(Decimal::ONE);
                true
            }
        }
    }
}

//

pub trait PerceptionMontoBaseRule {
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> bool;
}

impl<T> PerceptionMontoBaseRule for T
where
    T: InvoicePercepcionMontoBaseGetter + InvoicePercepcionMontoBaseSetter,
{
    fn summary(&mut self, total_importe: &TotalImporteInvoice) -> bool {
        match &self.get_montobase() {
            Some(_) => false,
            None => {
                self.set_montobase(total_importe.importe_sin_impuestos);
                true
            }
        }
    }
}

//

pub trait PerceptionMontoRule {
    fn summary(&mut self) -> bool;
}

impl<T> PerceptionMontoRule for T
where
    T: InvoicePercepcionMontoGetter
        + InvoicePercepcionMontoSetter
        + InvoicePercepcionMontoBaseGetter
        + InvoicePercepcionPorcentajeGetter,
{
    fn summary(&mut self) -> bool {
        match (
            self.get_monto(),
            self.get_montobase(),
            self.get_porcentaje(),
        ) {
            (None, Some(monto_base), Some(porcentaje)) => {
                self.set_monto(monto_base * porcentaje);
                false
            }
            _ => false,
        }
    }
}

//

pub trait PerceptionMontoTotalRule {
    fn summary(&mut self) -> bool;
}

impl<T> PerceptionMontoTotalRule for T
where
    T: InvoicePercepcionMontoTotalGetter
        + InvoicePercepcionMontoTotalSetter
        + InvoicePercepcionMontoBaseGetter
        + InvoicePercepcionMontoGetter,
{
    fn summary(&mut self) -> bool {
        match (
            self.get_montototal(),
            self.get_montobase(),
            self.get_monto(),
        ) {
            (None, Some(monto_total), Some(monto)) => {
                self.set_montototal(monto_total + monto);
                false
            }
            _ => false,
        }
    }
}
