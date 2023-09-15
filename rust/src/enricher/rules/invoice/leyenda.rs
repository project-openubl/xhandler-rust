use crate::catalogs::{Catalog, Catalog52, Label};
use crate::models::traits::invoice::detraccion::DetraccionGetter;
use crate::models::traits::leyendas::{LeyendasGetter, LeyendasSetter};

pub trait LeyendaDetraccionRule {
    fn enrich_leyenda_detraccion(&mut self) -> bool;
}

impl<T> LeyendaDetraccionRule for T
where
    T: DetraccionGetter + LeyendasGetter +  LeyendasSetter,
{
    fn enrich_leyenda_detraccion(&mut self) -> bool {
        match &self.get_detraccion() {
            Some(..) => false,
            None => {
                let catalog = &Catalog52::OperacionSujetaADetraccion;

                if self.get_leyendas().contains_key(catalog.code()) {
                    self.insert_leyendas(catalog.code(), catalog.label());
                    true
                } else {
                    false
                }
            }
        }
    }
}
