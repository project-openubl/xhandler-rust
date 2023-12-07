use rust_decimal::Decimal;

use crate::models::common::Percepcion;
use crate::models::invoice::Invoice;

pub trait InvoicePercepcionGetter {
    fn get_percepcion(&mut self) -> &mut Option<Percepcion>;
}

impl InvoicePercepcionGetter for Invoice {
    fn get_percepcion(&mut self) -> &mut Option<Percepcion> {
        &mut self.percepcion
    }
}

// Monto Base

pub trait InvoicePercepcionPorcentajeGetter {
    fn get_porcentaje(&self) -> &Option<Decimal>;
}

pub trait InvoicePercepcionPorcentajeSetter {
    fn set_porcentaje(&mut self, val: Decimal);
}

impl InvoicePercepcionPorcentajeGetter for Percepcion {
    fn get_porcentaje(&self) -> &Option<Decimal> {
        &self.porcentaje
    }
}

impl InvoicePercepcionPorcentajeSetter for Percepcion {
    fn set_porcentaje(&mut self, val: Decimal) {
        self.porcentaje = Some(val);
    }
}

// Monto Base

pub trait InvoicePercepcionMontoBaseGetter {
    fn get_montobase(&self) -> &Option<Decimal>;
}

pub trait InvoicePercepcionMontoBaseSetter {
    fn set_montobase(&mut self, val: Decimal);
}

impl InvoicePercepcionMontoBaseGetter for Percepcion {
    fn get_montobase(&self) -> &Option<Decimal> {
        &self.monto_base
    }
}

impl InvoicePercepcionMontoBaseSetter for Percepcion {
    fn set_montobase(&mut self, val: Decimal) {
        self.monto_base = Some(val);
    }
}

// Monto

pub trait InvoicePercepcionMontoGetter {
    fn get_monto(&self) -> &Option<Decimal>;
}

pub trait InvoicePercepcionMontoSetter {
    fn set_monto(&mut self, val: Decimal);
}

impl InvoicePercepcionMontoGetter for Percepcion {
    fn get_monto(&self) -> &Option<Decimal> {
        &self.monto
    }
}

impl InvoicePercepcionMontoSetter for Percepcion {
    fn set_monto(&mut self, val: Decimal) {
        self.monto = Some(val);
    }
}

// Monto total

pub trait InvoicePercepcionMontoTotalGetter {
    fn get_montototal(&self) -> &Option<Decimal>;
}

pub trait InvoicePercepcionMontoTotalSetter {
    fn set_montototal(&mut self, val: Decimal);
}

impl InvoicePercepcionMontoTotalGetter for Percepcion {
    fn get_montototal(&self) -> &Option<Decimal> {
        &self.monto_total
    }
}

impl InvoicePercepcionMontoTotalSetter for Percepcion {
    fn set_montototal(&mut self, val: Decimal) {
        self.monto_total = Some(val);
    }
}
