use crate::catalogs::{Catalog, Catalog51};
use crate::enricher::bounds::invoice::detraccion::InvoiceDetraccionGetter;
use crate::enricher::bounds::invoice::percepcion::InvoicePercepcionGetter;
use crate::enricher::bounds::invoice::tipo_operacion::{
    InvoiceTipoOperacionGetter, InvoiceTipoOperacionSetter,
};

pub trait InvoiceTipoOperacionEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> InvoiceTipoOperacionEnrichRule for T
where
    T: InvoiceTipoOperacionGetter
        + InvoiceTipoOperacionSetter
        + InvoiceDetraccionGetter
        + InvoicePercepcionGetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_tipo_operacion() {
            Some(..) => false,
            None => {
                if self.get_detraccion().is_some() {
                    self.set_tipo_operacion(Catalog51::OperacionSujetaADetraccion.code());
                } else if self.get_percepcion().is_some() {
                    self.set_tipo_operacion(Catalog51::OperacionSujetaAPercepcion.code());
                } else {
                    self.set_tipo_operacion(Catalog51::VentaInterna.code());
                }

                false
            }
        }
    }
}
