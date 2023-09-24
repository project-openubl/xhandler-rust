use crate::catalogs::{Catalog, Catalog52, Label};
use crate::models::traits::invoice::detraccion::InvoiceDetraccionGetter;
use crate::models::traits::invoice::direccionentrega::InvoiceDireccionEntregaGetter;
use crate::models::traits::invoice::percepcion::InvoicePercepcionGetter;
use crate::models::traits::leyendas::{LeyendasGetter, LeyendasSetter};

pub trait InvoiceLeyendaDetraccionEnrichRule {
    fn enrich(&mut self) -> bool;
}

pub trait InvoiceLeyendaDireccionEntregaEnrichRule {
    fn enrich(&mut self) -> bool;
}

pub trait InvoiceLeyendaPercepcionEnrichRule {
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

impl<T> InvoiceLeyendaDetraccionEnrichRule for T
where
    T: InvoiceDetraccionGetter + LeyendasGetter + LeyendasSetter,
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

impl<T> InvoiceLeyendaDireccionEntregaEnrichRule for T
where
    T: InvoiceDireccionEntregaGetter + LeyendasGetter + LeyendasSetter,
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

impl<T> InvoiceLeyendaPercepcionEnrichRule for T
where
    T: InvoicePercepcionGetter + LeyendasGetter + LeyendasSetter,
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
