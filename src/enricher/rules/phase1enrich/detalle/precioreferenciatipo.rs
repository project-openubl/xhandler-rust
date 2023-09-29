use crate::catalogs::{catalog7_value_of_code, Catalog, Catalog16};
use crate::enricher::rules::phase1enrich::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::precioreferenciatipo::{
    DetallePrecioReferenciaTipoGetter, DetallePrecioReferenciaTipoSetter,
};

pub trait DetallePrecioReferenciaTipoEnrichRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetallePrecioReferenciaTipoEnrichRule for T
where
    T: DetallePrecioReferenciaTipoGetter + DetallePrecioReferenciaTipoSetter + DetalleIGVTipoGetter,
{
    fn enrich(&mut self, _: &DetalleDefaults) -> bool {
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
