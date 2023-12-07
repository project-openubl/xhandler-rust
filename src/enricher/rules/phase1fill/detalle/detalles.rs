use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::icb::IcbTasaGetter;
use crate::enricher::bounds::igv::IgvTasaGetter;
use crate::enricher::bounds::ivap::IvapTasaGetter;
use crate::enricher::rules::phase1fill::detalle::icb_tasa::DetalleICBTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::igv_tasa::DetalleIGVTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::igv_tipo::DetalleIGVTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::isc_tasa::DetalleISCTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::isc_tipo::DetalleISCTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::precio_referencia_tipo::DetallePrecioReferenciaTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::unidad_medida::DetalleUnidadMedidaEnrichRule;

pub struct DetalleDefaults {
    pub igv_tasa: Decimal,
    pub icb_tasa: Decimal,
    pub ivap_tasa: Decimal,
}

pub trait DetallesEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> DetallesEnrichRule for T
where
    T: DetallesGetter + IgvTasaGetter + IcbTasaGetter + IvapTasaGetter,
{
    fn fill(&mut self) -> bool {
        let defaults = &DetalleDefaults {
            igv_tasa: self.get_igv_tasa().expect("IGV Tasa could not be found"),
            icb_tasa: self.get_icb_tasa().expect("IBC Tasa could not be found"),
            ivap_tasa: self.get_ivap_tasa().expect("IVAP Tasa could not be found"),
        };

        self.get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = [
                    DetalleICBTasaEnrichRule::fill(detalle, defaults),
                    DetalleIGVTasaEnrichRule::fill(detalle, defaults),
                    DetalleIGVTipoEnrichRule::fill(detalle, defaults),
                    DetalleISCTasaEnrichRule::fill(detalle, defaults),
                    DetalleISCTipoEnrichRule::fill(detalle, defaults),
                    DetallePrecioReferenciaTipoEnrichRule::fill(detalle, defaults),
                    DetalleUnidadMedidaEnrichRule::fill(detalle, defaults),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
