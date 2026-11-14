use anyhow::Result;

use crate::xbuilder::catalogs::{Catalog, Catalog52, Label};
use crate::xbuilder::enricher::bounds::invoice::detraccion::InvoiceDetraccionGetter;
use crate::xbuilder::enricher::bounds::invoice::direccion_entrega::InvoiceDireccionEntregaGetter;
use crate::xbuilder::enricher::bounds::invoice::percepcion::InvoicePercepcionGetter;
use crate::xbuilder::enricher::bounds::leyendas::{LeyendasGetter, LeyendasSetter};
use crate::xbuilder::enricher::rules::insert_leyenda_if_absent;

pub trait InvoiceLeyendaDetraccionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceLeyendaDireccionEntregaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait InvoiceLeyendaPercepcionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceLeyendaDetraccionFillRule for T
where
    T: InvoiceDetraccionGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.detraccion() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::OperacionSujetaADetraccion;
                Ok(insert_leyenda_if_absent(
                    self,
                    catalog.code(),
                    catalog.label(),
                ))
            }
        }
    }
}

impl<T> InvoiceLeyendaDireccionEntregaFillRule for T
where
    T: InvoiceDireccionEntregaGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.direccion_entrega() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::VentaRealizadaPorEmisorItinerante;
                Ok(insert_leyenda_if_absent(
                    self,
                    catalog.code(),
                    catalog.label(),
                ))
            }
        }
    }
}

impl<T> InvoiceLeyendaPercepcionFillRule for T
where
    T: InvoicePercepcionGetter + LeyendasGetter + LeyendasSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.percepcion() {
            None => Ok(false),
            Some(..) => {
                let catalog = &Catalog52::ComprobanteDePercepcion;
                Ok(insert_leyenda_if_absent(
                    self,
                    catalog.code(),
                    catalog.label(),
                ))
            }
        }
    }
}
