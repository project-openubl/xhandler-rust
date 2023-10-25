use crate::catalogs::{Catalog, Catalog16, catalog7_value_of_code};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::precioreferenciatipo::{
    DetallePrecioReferenciaTipoGetter, DetallePrecioReferenciaTipoSetter,
};

pub trait DetallePrecioReferenciaTipoEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetallePrecioReferenciaTipoEnrichRule for T
where
    T: DetallePrecioReferenciaTipoGetter + DetallePrecioReferenciaTipoSetter + DetalleIGVTipoGetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match (self.get_precioreferenciatipo(), *self.get_igvtipo()) {
            (None, Some(igv_tipo)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let catalog16 = if catalog.onerosa() {
                        &Catalog16::PrecioUnitarioIncluyeIgv
                    } else {
                        &Catalog16::ValorReferencialUnitarioEnOperacionesNoOnerosas
                    };
                    self.set_precioreferenciatipo(catalog16.code());
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
