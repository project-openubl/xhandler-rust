use anyhow::Result;

use crate::catalogs::{Catalog, Catalog7};
use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::leyendas::{LeyendasGetter, LeyendasSetter};
use crate::enricher::rules::insert_leyenda_if_absent;

pub trait LeyendaIvapSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> LeyendaIvapSummaryRule for T
where
    T: DetallesGetter + LeyendasGetter + LeyendasSetter,
{
    fn summary(&mut self) -> Result<bool> {
        if self
            .detalles()
            .iter()
            .any(|e| e.igv_tipo.as_deref().unwrap_or("") == Catalog7::GravadoIvap.code())
        {
            Ok(insert_leyenda_if_absent(
                self,
                "2007",
                "Leyenda: Operacion sujeta a IVAP",
            ))
        } else {
            Ok(false)
        }
    }
}
