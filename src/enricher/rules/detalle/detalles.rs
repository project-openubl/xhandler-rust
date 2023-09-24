use crate::enricher::rules::detalle::icbtasa::DetalleICBTasaRule;
use crate::enricher::rules::detalle::igvtasa::DetalleIGVTasaRule;
use crate::enricher::rules::detalle::igvtipo::DetalleIGVTipoRule;
use crate::enricher::rules::detalle::isctasa::DetalleISCTasaRule;
use crate::enricher::rules::detalle::isctipo::DetalleISCTipoRule;
use crate::enricher::rules::detalle::precioreferenciatipo::DetallePrecioReferenciaTipoRule;
use crate::enricher::rules::detalle::unidadmedida::DetalleUnidadMedidaRule;
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::icb::ICBTasaGetter;
use crate::models::traits::igv::IGVTasaGetter;
use crate::models::traits::ivap::IVAPTasaGetter;

pub struct DetalleDefaults {
    pub igv_tasa: f32,
    pub icb_tasa: f32,
    pub ivap_tasa: f32,
}

pub trait DetallesRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DetallesRule for T
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
                    DetalleICBTasaRule::enrich(detalle, defaults),
                    DetalleIGVTasaRule::enrich(detalle, defaults),
                    DetalleIGVTipoRule::enrich(detalle, defaults),
                    DetalleISCTasaRule::enrich(detalle, defaults),
                    DetalleISCTipoRule::enrich(detalle, defaults),
                    DetallePrecioReferenciaTipoRule::enrich(detalle, defaults),
                    DetalleUnidadMedidaRule::enrich(detalle, defaults),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
