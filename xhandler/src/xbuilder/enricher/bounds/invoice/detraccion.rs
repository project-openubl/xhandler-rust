use rust_decimal::Decimal;

use crate::xbuilder::models::common::Detraccion;
use crate::xbuilder::models::invoice::Invoice;

pub trait InvoiceDetraccionGetter {
    fn detraccion(&self) -> &Option<Detraccion>;
    fn detraccion_mut(&mut self) -> &mut Option<Detraccion>;
}

impl InvoiceDetraccionGetter for Invoice {
    fn detraccion(&self) -> &Option<Detraccion> {
        &self.detraccion
    }

    fn detraccion_mut(&mut self) -> &mut Option<Detraccion> {
        &mut self.detraccion
    }
}

// Monto

pub trait DetraccionMontoGetter {
    fn monto(&self) -> &Option<Decimal>;
}

pub trait DetraccionMontoSetter {
    fn set_monto(&mut self, val: Decimal);
}

impl DetraccionMontoGetter for Detraccion {
    fn monto(&self) -> &Option<Decimal> {
        &self.monto
    }
}

impl DetraccionMontoSetter for Detraccion {
    fn set_monto(&mut self, val: Decimal) {
        self.monto = Some(val);
    }
}

// Porcentaje

// Monto

pub trait DetraccionPorcentajeGetter {
    fn porcentaje(&self) -> &Decimal;
}

impl DetraccionPorcentajeGetter for Detraccion {
    fn porcentaje(&self) -> &Decimal {
        &self.porcentaje
    }
}
