use crate::catalogs::{Catalog, Catalog52, Label};
use crate::models::traits::invoice::detraccion::DetraccionGetter;
use crate::models::traits::invoice::direccionentrega::DireccionEntregaGetter;
use crate::models::traits::invoice::percepcion::PercepcionGetter;
use crate::models::traits::leyendas::{LeyendasGetter, LeyendasSetter};

pub trait InvoiceLeyendaDetraccionRule {
    fn enrich(&mut self) -> bool;
}

pub trait InvoiceLeyendaDireccionEntregaRule {
    fn enrich(&mut self) -> bool;
}

pub trait InvoiceLeyendaPercepcionRule {
    fn enrich(&mut self) -> bool;
}

fn insert_leyenda<T>(obj: &mut T, code: &'static str, label: &'static str) -> bool
where
    T: LeyendasGetter + LeyendasSetter,
{
    if obj.get_leyendas().contains_key(code) {
        obj.insert_leyendas(code, label);
        true
    } else {
        false
    }
}

impl<T> InvoiceLeyendaDetraccionRule for T
where
    T: DetraccionGetter + LeyendasGetter + LeyendasSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_detraccion() {
            Some(..) => false,
            None => {
                let catalog = &Catalog52::OperacionSujetaADetraccion;
                insert_leyenda(self, catalog.code(), catalog.label())
            }
        }
    }
}

impl<T> InvoiceLeyendaDireccionEntregaRule for T
where
    T: DireccionEntregaGetter + LeyendasGetter + LeyendasSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_direccionentrega() {
            Some(..) => false,
            None => {
                let catalog = &Catalog52::VentaRealizadaPorEmisorItinerante;
                insert_leyenda(self, catalog.code(), catalog.label())
            }
        }
    }
}

impl<T> InvoiceLeyendaPercepcionRule for T
where
    T: PercepcionGetter + LeyendasGetter + LeyendasSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_percepcion() {
            Some(..) => false,
            None => {
                let catalog = &Catalog52::ComprobanteDePercepcion;
                insert_leyenda(self, catalog.code(), catalog.label())
            }
        }
    }
}
