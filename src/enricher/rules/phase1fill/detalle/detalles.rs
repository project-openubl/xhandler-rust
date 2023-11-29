use crate::enricher::rules::phase1fill::detalle::icbtasa::DetalleICBTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::igvtasa::DetalleIGVTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::igvtipo::DetalleIGVTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::isctasa::DetalleISCTasaEnrichRule;
use crate::enricher::rules::phase1fill::detalle::isctipo::DetalleISCTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::precioreferenciatipo::DetallePrecioReferenciaTipoEnrichRule;
use crate::enricher::rules::phase1fill::detalle::unidadmedida::DetalleUnidadMedidaEnrichRule;
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::icb::ICBTasaGetter;
use crate::models::traits::igv::IGVTasaGetter;
use crate::models::traits::ivap::IVAPTasaGetter;
use rust_decimal::Decimal;

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
    T: DetallesGetter + IGVTasaGetter + ICBTasaGetter + IVAPTasaGetter,
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
                let results = vec![
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
