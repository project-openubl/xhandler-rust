use crate::models::general::Percepcion;
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

pub trait InvoicePercepcionMontoBaseGetter {
    fn get_montobase(&self) -> &Option<f64>;
}

pub trait InvoicePercepcionMontoBaseSetter {
    fn set_montobase(&mut self, val: f64);
}

impl InvoicePercepcionMontoBaseGetter for Percepcion {
    fn get_montobase(&self) -> &Option<f64> {
        &self.monto_base
    }
}

impl InvoicePercepcionMontoBaseSetter for Percepcion {
    fn set_montobase(&mut self, val: f64) {
        self.monto_base = Some(val);
    }
}

// Monto

pub trait InvoicePercepcionMontoGetter {
    fn get_monto(&self) -> &Option<f64>;
}

pub trait InvoicePercepcionMontoSetter {
    fn set_monto(&mut self, val: f64);
}

impl InvoicePercepcionMontoGetter for Percepcion {
    fn get_monto(&self) -> &Option<f64> {
        &self.monto
    }
}

impl InvoicePercepcionMontoSetter for Percepcion {
    fn set_monto(&mut self, val: f64) {
        self.monto = Some(val);
    }
}

// Monto total

pub trait InvoicePercepcionMontoTotalGetter {
    fn get_montototal(&self) -> &Option<f64>;
}

pub trait InvoicePercepcionMontoTotalSetter {
    fn set_montototal(&mut self, val: f64);
}

impl InvoicePercepcionMontoTotalGetter for Percepcion {
    fn get_montototal(&self) -> &Option<f64> {
        &self.monto_total
    }
}

impl InvoicePercepcionMontoTotalSetter for Percepcion {
    fn set_montototal(&mut self, val: f64) {
        self.monto_total = Some(val);
    }
}
