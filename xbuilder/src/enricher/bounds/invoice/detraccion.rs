use rust_decimal::Decimal;

use crate::models::common::Detraccion;
use crate::models::invoice::Invoice;

pub trait InvoiceDetraccionGetter {
    fn get_detraccion(&mut self) -> &mut Option<Detraccion>;
}

impl InvoiceDetraccionGetter for Invoice {
    fn get_detraccion(&mut self) -> &mut Option<Detraccion> {
        &mut self.detraccion
    }
}

// Monto

pub trait InvoiceDetraccionMontoGetter {
    fn get_monto(&self) -> &Option<Decimal>;
}

pub trait InvoiceDetraccionMontoSetter {
    fn set_monto(&mut self, val: Decimal);
}

impl InvoiceDetraccionMontoGetter for Detraccion {
    fn get_monto(&self) -> &Option<Decimal> {
        &self.monto
    }
}

impl InvoiceDetraccionMontoSetter for Detraccion {
    fn set_monto(&mut self, val: Decimal) {
        self.monto = Some(val);
    }
}

// Porcentaje

// Monto

pub trait InvoiceDetraccionPorcentajeGetter {
    fn get_porcentaje(&self) -> &Decimal;
}

impl InvoiceDetraccionPorcentajeGetter for Detraccion {
    fn get_porcentaje(&self) -> &Decimal {
        &self.porcentaje
    }
}
