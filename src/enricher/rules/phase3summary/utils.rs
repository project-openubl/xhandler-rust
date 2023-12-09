use rust_decimal::Decimal;

use crate::catalogs::{Catalog5, Catalog7, FromCode};
use crate::models::common::Detalle;

pub struct Impuesto {
    pub base_imponible: Decimal,

    pub importe: Decimal,
    pub importe_igv: Decimal,
    pub importe_isc: Decimal,
    pub importe_icb: Decimal,
}

pub fn cal_impuesto_by_tipo(detalles: &[Detalle], categoria: Catalog5) -> Impuesto {
    let stream: Vec<&Detalle> = detalles
        .iter()
        .filter(|e| e.igv_tipo.is_some() && e.igv_tipo.is_some())
        .filter(|e| {
            if let Ok(catalog7) = Catalog7::from_code(e.igv_tipo.unwrap()) {
                categoria == catalog7.tax_category()
            } else {
                false
            }
        })
        .collect();

    let base_imponible = stream
        .iter()
        .map(|e| e.isc_base_imponible)
        .filter(|e| e.is_some())
        .fold(Decimal::ZERO, |a, b| a + b.unwrap());
    let importe = stream
        .iter()
        .map(|e| e.total_impuestos)
        .filter(|e| e.is_some())
        .fold(Decimal::ZERO, |a, b| a + b.unwrap());

    let importe_isc = stream
        .iter()
        .map(|e| e.isc)
        .filter(|e| e.is_some())
        .fold(Decimal::ZERO, |a, b| a + b.unwrap());
    let importe_igv = stream
        .iter()
        .map(|e| e.igv)
        .filter(|e| e.is_some())
        .fold(Decimal::ZERO, |a, b| a + b.unwrap());
    let importe_icb = stream
        .iter()
        .map(|e| e.icb)
        .filter(|e| e.is_some())
        .fold(Decimal::ZERO, |a, b| a + b.unwrap());

    Impuesto {
        base_imponible,
        importe,
        importe_igv,
        importe_isc,
        importe_icb,
    }
}

pub fn importe_sin_impuestos(detalle: &[Detalle]) -> Decimal {
    detalle
        .iter()
        .filter(|e| {
            if let Ok(catalog7) = Catalog7::from_code(e.igv_tipo.unwrap_or("")) {
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
