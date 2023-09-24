use crate::enricher::rules::phase1enrich::detalle::icbtasa::DetalleICBTasaEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::igvtasa::DetalleIGVTasaEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::igvtipo::DetalleIGVTipoEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::isctasa::DetalleISCTasaEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::isctipo::DetalleISCTipoEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::precioreferenciatipo::DetallePrecioReferenciaTipoEnrichRule;
use crate::enricher::rules::phase1enrich::detalle::unidadmedida::DetalleUnidadMedidaEnrichRule;
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::icb::ICBTasaGetter;
use crate::models::traits::igv::IGVTasaGetter;
use crate::models::traits::ivap::IVAPTasaGetter;

pub struct DetalleDefaults {
    pub igv_tasa: f32,
    pub icb_tasa: f32,
    pub ivap_tasa: f32,
}

pub trait DetallesEnrichRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DetallesEnrichRule for T
where
    T: DetallesGetter + IGVTasaGetter + ICBTasaGetter + IVAPTasaGetter,
{
    fn enrich(&mut self) -> bool {
        let defaults = &DetalleDefaults {
            igv_tasa: self.get_igv_tasa().expect("IGV Tasa could not be found"),
            icb_tasa: self.get_icb_tasa().expect("IBC Tasa could not be found"),
            ivap_tasa: self.get_ivap_tasa().expect("IVAP Tasa could not be found"),
        };

        self.get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = vec![
                    DetalleICBTasaEnrichRule::enrich(detalle, defaults),
                    DetalleIGVTasaEnrichRule::enrich(detalle, defaults),
                    DetalleIGVTipoEnrichRule::enrich(detalle, defaults),
                    DetalleISCTasaEnrichRule::enrich(detalle, defaults),
                    DetalleISCTipoEnrichRule::enrich(detalle, defaults),
                    DetallePrecioReferenciaTipoEnrichRule::enrich(detalle, defaults),
                    DetalleUnidadMedidaEnrichRule::enrich(detalle, defaults),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
