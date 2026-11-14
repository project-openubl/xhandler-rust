use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{Catalog5, Catalog7, FromCode};
use crate::xbuilder::models::common::Detalle;

pub struct Impuesto {
    pub base_imponible: Decimal,

    pub importe: Decimal,
    pub igv_importe: Decimal,
    pub isc_importe: Decimal,
    pub icb_importe: Decimal,
}

pub fn compute_impuesto_by_tipo(detalles: &[Detalle], categoria: Catalog5) -> Impuesto {
    let stream: Vec<&Detalle> = detalles
        .iter()
        .filter(|e| {
            if let Some(igv_tipo) = e.igv_tipo.as_deref() {
                if let Ok(catalog7) = Catalog7::from_code(igv_tipo) {
                    return categoria == catalog7.tax_category();
                }
            }
            false
        })
        .collect();

    let base_imponible = stream
        .iter()
        .filter_map(|e| e.isc_base_imponible)
        .fold(Decimal::ZERO, |a, b| a + b);
    let importe = stream
        .iter()
        .filter_map(|e| e.total_impuestos)
        .fold(Decimal::ZERO, |a, b| a + b);

    let isc_importe = stream
        .iter()
        .filter_map(|e| e.isc)
        .fold(Decimal::ZERO, |a, b| a + b);
    let igv_importe = stream
        .iter()
        .filter_map(|e| e.igv)
        .fold(Decimal::ZERO, |a, b| a + b);
    let icb_importe = stream
        .iter()
        .filter_map(|e| e.icb)
        .fold(Decimal::ZERO, |a, b| a + b);

    Impuesto {
        base_imponible,
        importe,
        igv_importe,
        isc_importe,
        icb_importe,
    }
}

pub fn importe_sin_impuestos(detalle: &[Detalle]) -> Decimal {
    detalle
        .iter()
        .filter(|e| {
            if let Ok(catalog7) = Catalog7::from_code(e.igv_tipo.as_deref().unwrap_or("")) {
                catalog7.tax_category() != Catalog5::Gratuito
            } else {
                false
            }
        })
        .filter_map(|detalle| {
            if detalle
                .isc_base_imponible
                .is_some_and(|base_imponible| base_imponible > Decimal::ZERO)
            {
                detalle.isc_base_imponible
            } else {
                detalle.igv_base_imponible
            }
        })
        .fold(Decimal::ZERO, |a, b| a + b)
}

pub fn total_impuestos(detalle: &[Detalle]) -> Decimal {
    detalle
        .iter()
        .filter_map(|e| e.total_impuestos)
        .fold(Decimal::ZERO, |a, b| a + b)
}
