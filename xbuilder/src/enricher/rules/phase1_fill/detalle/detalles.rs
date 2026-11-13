use anyhow::{anyhow, Result};
use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::icb_tasa::IcbTasaGetter;
use crate::enricher::bounds::igv_tasa::IgvTasaGetter;
use crate::enricher::bounds::ivap_tasa::IvapTasaGetter;
use crate::enricher::rules::phase1_fill::detalle::icb_tasa::DetalleIcbTasaFillRule;
use crate::enricher::rules::phase1_fill::detalle::igv_tasa::DetalleIgvTasaFillRule;
use crate::enricher::rules::phase1_fill::detalle::igv_tipo::DetalleIgvTipoFillRule;
use crate::enricher::rules::phase1_fill::detalle::isc_tasa::DetalleIscTasaFillRule;
use crate::enricher::rules::phase1_fill::detalle::isc_tipo::DetalleIscTipoFillRule;
use crate::enricher::rules::phase1_fill::detalle::precio_referencia_tipo::DetallePrecioReferenciaTipoFillRule;
use crate::enricher::rules::phase1_fill::detalle::unidad_medida::DetalleUnidadMedidaFillRule;

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
                .igv_tasa()
                .ok_or(anyhow!("IGV Tasa could not be found"))?,
            icb_tasa: self
                .icb_tasa()
                .ok_or(anyhow!("ICB Tasa could not be found"))?,
            ivap_tasa: self
                .ivap_tasa()
                .ok_or(anyhow!("IVAP Tasa could not be found"))?,
        };

        let result = self.detalles_mut().iter_mut().any(|detalle| {
            let results = [
                DetalleIcbTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetalleIgvTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetalleIgvTipoFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetalleIscTasaFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetalleIscTipoFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetallePrecioReferenciaTipoFillRule::fill(detalle, defaults).unwrap_or_default(),
                DetalleUnidadMedidaFillRule::fill(detalle, defaults).unwrap_or_default(),
            ];
            results.contains(&true)
        });
        Ok(result)
    }
}
