use anyhow::Result;

use crate::catalogs::{Catalog, Catalog52, Label};
use crate::enricher::bounds::invoice::detraccion::InvoiceDetraccionGetter;
use crate::enricher::bounds::invoice::direccion_entrega::InvoiceDireccionEntregaGetter;
use crate::enricher::bounds::invoice::percepcion::InvoicePercepcionGetter;
use crate::enricher::bounds::leyendas::{LeyendasGetter, LeyendasSetter};

pub trait InvoiceLeyendaDetraccionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceLeyendaDireccionEntregaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceLeyendaPercepcionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

fn insert_leyenda<T>(obj: &mut T, code: &'static str, label: &'static str) -> bool
where
    T: LeyendasGetter + LeyendasSetter,
{
    if !obj.get_leyendas().contains_key(code) {
        obj.insert_leyendas(code, label);
        true
    } else {
        false
    }
}

impl<T> InvoiceLeyendaDetraccionFillRule for T
where
    T: InvoiceDetraccionGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_detraccion() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::OperacionSujetaADetraccion;
                Ok(insert_leyenda(self, catalog.code(), catalog.label()))
            }
        }
    }
}

impl<T> InvoiceLeyendaDireccionEntregaFillRule for T
where
    T: InvoiceDireccionEntregaGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_direccion_entrega() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::VentaRealizadaPorEmisorItinerante;
                Ok(insert_leyenda(self, catalog.code(), catalog.label()))
            }
        }
    }
}

impl<T> InvoiceLeyendaPercepcionFillRule for T
where
    T: InvoicePercepcionGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_percepcion() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::ComprobanteDePercepcion;
                Ok(insert_leyenda(self, catalog.code(), catalog.label()))
            }
        }
    }
}
