use anyhow::Result;

use crate::xbuilder::catalogs::{Catalog, Catalog51};
use crate::xbuilder::enricher::bounds::invoice::detraccion::InvoiceDetraccionGetter;
use crate::xbuilder::enricher::bounds::invoice::percepcion::InvoicePercepcionGetter;
use crate::xbuilder::enricher::bounds::invoice::tipo_operacion::{
    InvoiceTipoOperacionGetter, InvoiceTipoOperacionSetter,
};

pub trait InvoiceTipoOperacionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceTipoOperacionFillRule for T
where
    T: InvoiceTipoOperacionGetter
        + InvoiceTipoOperacionSetter
        + InvoiceDetraccionGetter
        + InvoicePercepcionGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.tipo_operacion() {
            Some(..) => Ok(false),
            None => {
                if self.detraccion().is_some() {
                    self.set_tipo_operacion(Catalog51::OperacionSujetaADetraccion.code());
                } else if self.percepcion().is_some() {
                    self.set_tipo_operacion(Catalog51::OperacionSujetaAPercepcion.code());
                } else {
                    self.set_tipo_operacion(Catalog51::VentaInterna.code());
                }

                Ok(false)
            }
        }
    }
}
