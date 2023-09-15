use crate::catalogs::{Catalog, Catalog51};
use crate::models::traits::invoice::detraccion::DetraccionGetter;
use crate::models::traits::invoice::percepcion::PercepcionGetter;
use crate::models::traits::invoice::tipooperacion::{TipoOperacionGetter, TipoOperacionSetter};

pub trait TipoOperacionRule {
    fn enrich_tipooperacion(&mut self) -> bool;
}

impl<T> TipoOperacionRule for T
where
    T: TipoOperacionGetter + TipoOperacionSetter + DetraccionGetter + PercepcionGetter,
{
    fn enrich_tipooperacion(&mut self) -> bool {
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
