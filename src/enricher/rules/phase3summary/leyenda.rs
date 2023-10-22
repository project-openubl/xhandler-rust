use crate::catalogs::Catalog;
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::leyendas::{LeyendasGetter, LeyendasSetter};
use crate::prelude::Catalog7;

pub trait LeyendaIVAPSummaryRule {
    fn summary(&mut self) -> bool;
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

impl<T> LeyendaIVAPSummaryRule for T
where
    T: DetallesGetter + LeyendasGetter + LeyendasSetter,
{
    fn summary(&mut self) -> bool {
        if self
            .get_detalles()
            .iter()
            .any(|e| e.igv_tipo.unwrap_or("") == Catalog7::GravadoIvap.code())
        {
            insert_leyenda(self, "2007", "Leyenda: Operacion sujeta a IVAP")
        } else {
            false
        }
    }
}
