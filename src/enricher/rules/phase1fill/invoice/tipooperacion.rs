use crate::catalogs::{Catalog, Catalog51};
use crate::models::traits::invoice::detraccion::InvoiceDetraccionGetter;
use crate::models::traits::invoice::percepcion::InvoicePercepcionGetter;
use crate::models::traits::invoice::tipooperacion::{
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
        match &self.get_tipooperacion() {
            Some(..) => false,
            None => {
                if self.get_detraccion().is_some() {
                    self.set_tipooperacion(Catalog51::OperacionSujetaADetraccion.code());
                } else if self.get_percepcion().is_some() {
                    self.set_tipooperacion(Catalog51::OperacionSujetaAPercepcion.code());
                } else {
                    self.set_tipooperacion(Catalog51::VentaInterna.code());
                }

                false
            }
        }
    }
}
