use anyhow::Result;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::note::total_importe::{
    NoteTotalImporteGetter, NoteTotalImporteSetter,
};
use crate::enricher::rules::phase3summary::utils::{importe_sin_impuestos, total_impuestos};
use crate::models::common::TotalImporteNote;

pub trait NoteTotalImporteSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> NoteTotalImporteSummaryRule for T
where
    T: NoteTotalImporteGetter + NoteTotalImporteSetter + DetallesGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.get_total_importe() {
            Some(..) => Ok(false),
            None => {
                let total_impuestos = total_impuestos(self.get_detalles());
                let importe_sin_impuestos = importe_sin_impuestos(self.get_detalles());

                let importe = importe_sin_impuestos + total_impuestos;

                self.set_total_importe(TotalImporteNote {
                    importe_sin_impuestos,
                    importe,
                });
                Ok(true)
            }
        }
    }
}
