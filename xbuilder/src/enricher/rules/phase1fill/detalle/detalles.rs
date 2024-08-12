use anyhow::{anyhow, Result};
use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::icb::IcbTasaGetter;
use crate::enricher::bounds::igv::IgvTasaGetter;
use crate::enricher::bounds::ivap::IvapTasaGetter;
use crate::enricher::rules::phase1fill::detalle::icb_tasa::DetalleICBTasaFillRule;
use crate::enricher::rules::phase1fill::detalle::igv_tasa::DetalleIGVTasaFillRule;
use crate::enricher::rules::phase1fill::detalle::igv_tipo::DetalleIGVTipoFillRule;
use crate::enricher::rules::phase1fill::detalle::isc_tasa::DetalleISCTasaFillRule;
use crate::enricher::rules::phase1fill::detalle::isc_tipo::DetalleISCTipoFillRule;
use crate::enricher::rules::phase1fill::detalle::precio_referencia_tipo::DetallePrecioReferenciaTipoFillRule;
use crate::enricher::rules::phase1fill::detalle::unidad_medida::DetalleUnidadMedidaFillRule;

pub struct DetalleDefaults {
    pub igv_tasa: Decimal,
    pub icb_tasa: Decimal,
    pub ivap_tasa: Decimal,
}

pub trait DetallesFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DetallesFillRule for T
where
    T: DetallesGetter + IgvTasaGetter + IcbTasaGetter + IvapTasaGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let defaults = &DetalleDefaults {
            igv_tasa: self
                .get_igv_tasa()
                .ok_or(anyhow!("IGV Tasa could not be found"))?,
            icb_tasa: self
                .get_icb_tasa()
                .ok_or(anyhow!("IBC Tasa could not be found"))?,
            ivap_tasa: self
                .get_ivap_tasa()
                .ok_or(anyhow!("IVAP Tasa could not be found"))?,
        };

        let result = self
            .get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = [
                    DetalleICBTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                    DetalleIGVTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                    DetalleIGVTipoFillRule::fill(detalle, defaults).unwrap_or_default(),
                    DetalleISCTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                    DetalleISCTipoFillRule::fill(detalle, defaults).unwrap_or_default(),
                    DetallePrecioReferenciaTipoFillRule::fill(detalle, defaults)
                        .unwrap_or_default(),
                    DetalleUnidadMedidaFillRule::fill(detalle, defaults).unwrap_or_default(),
                ];
                results.contains(&true)
            })
            .any(|changed| changed);
        Ok(result)
    }
}
