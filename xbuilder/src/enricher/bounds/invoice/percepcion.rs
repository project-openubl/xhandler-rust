use rust_decimal::Decimal;

use crate::models::common::Percepcion;
use crate::models::invoice::Invoice;

pub trait InvoicePercepcionGetter {
    fn percepcion(&self) -> &Option<Percepcion>;
    fn percepcion_mut(&mut self) -> &mut Option<Percepcion>;
}

impl InvoicePercepcionGetter for Invoice {
    fn percepcion(&self) -> &Option<Percepcion> {
        &self.percepcion
    }

    fn percepcion_mut(&mut self) -> &mut Option<Percepcion> {
        &mut self.percepcion
    }
}

// Porcentaje

pub trait PercepcionPorcentajeGetter {
    fn porcentaje(&self) -> &Option<Decimal>;
}

pub trait PercepcionPorcentajeSetter {
    fn set_porcentaje(&mut self, val: Decimal);
}

impl PercepcionPorcentajeGetter for Percepcion {
    fn porcentaje(&self) -> &Option<Decimal> {
        &self.porcentaje
    }
}

impl PercepcionPorcentajeSetter for Percepcion {
    fn set_porcentaje(&mut self, val: Decimal) {
        self.porcentaje = Some(val);
    }
}

// Monto Base

pub trait PercepcionMontoBaseGetter {
    fn monto_base(&self) -> &Option<Decimal>;
}

pub trait PercepcionMontoBaseSetter {
    fn set_monto_base(&mut self, val: Decimal);
}

impl PercepcionMontoBaseGetter for Percepcion {
    fn monto_base(&self) -> &Option<Decimal> {
        &self.monto_base
    }
}

impl PercepcionMontoBaseSetter for Percepcion {
    fn set_monto_base(&mut self, val: Decimal) {
        self.monto_base = Some(val);
    }
}

// Monto

pub trait PercepcionMontoGetter {
    fn monto(&self) -> &Option<Decimal>;
}

pub trait PercepcionMontoSetter {
    fn set_monto(&mut self, val: Decimal);
}

impl PercepcionMontoGetter for Percepcion {
    fn monto(&self) -> &Option<Decimal> {
        &self.monto
    }
}

impl PercepcionMontoSetter for Percepcion {
    fn set_monto(&mut self, val: Decimal) {
        self.monto = Some(val);
    }
}

// Monto total

pub trait PercepcionMontoTotalGetter {
    fn monto_total(&self) -> &Option<Decimal>;
}

pub trait PercepcionMontoTotalSetter {
    fn set_monto_total(&mut self, val: Decimal);
}

impl PercepcionMontoTotalGetter for Percepcion {
    fn monto_total(&self) -> &Option<Decimal> {
        &self.monto_total
    }
}

impl PercepcionMontoTotalSetter for Percepcion {
    fn set_monto_total(&mut self, val: Decimal) {
        self.monto_total = Some(val);
    }
}
