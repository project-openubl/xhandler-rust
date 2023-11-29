use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::catalogs::{catalog53_value_of_code, catalog7_value_of_code, Catalog5, Catalog53};
use crate::models::general::{Detalle, TotalImpuestos};
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::invoice::anticipos::InvoiceAnticiposGetter;
use crate::models::traits::invoice::descuentos::InvoiceDescuentosGetter;
use crate::models::traits::totalimpuestos::{TotalImpuestosGetter, TotalImpuestosSetter};

pub trait InvoiceTotalImpuestosSummaryRule {
    fn summary(&mut self) -> bool;
}

impl<T> InvoiceTotalImpuestosSummaryRule for T
where
    T: TotalImpuestosGetter
        + TotalImpuestosSetter
        + DetallesGetter
        + InvoiceDescuentosGetter
        + InvoiceAnticiposGetter,
{
    fn summary(&mut self) -> bool {
        match &self.get_totalimpuestos() {
            Some(..) => false,
            None => {
                let ivap = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::ImpuestoArrozPilado);
                let exportacion = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Exportacion);
                let gravado = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Igv);
                let inafecto = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Inafecto);
                let exonerado = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Exonerado);
                let gratuito = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Gratuito);

                // ICB
                let icb_importe = vec![
                    ivap.importe_icb,
                    exportacion.importe_icb,
                    gravado.importe_icb,
                    inafecto.importe_icb,
                    exonerado.importe_icb,
                    gratuito.importe_icb,
                ]
                .iter()
                .fold(dec!(0), |a, b| a + b);

                // ISC
                let isc_importe = vec![
                    ivap.importe_isc,
                    exportacion.importe_isc,
                    gravado.importe_isc,
                    inafecto.importe_isc,
                    exonerado.importe_isc,
                    gratuito.importe_isc,
                ]
                .iter()
                .fold(dec!(0), |a, b| a + b);

                let isc_base_imponible = &self
                    .get_detalles()
                    .iter()
                    .filter(|e| {
                        if let Some(isc_tasa) = e.isc_tasa {
                            isc_tasa > dec!(0)
                        } else {
                            false
                        }
                    })
                    .filter_map(|e| e.isc_base_imponible)
                    .fold(dec!(0), |a, b| a + b);

                // Anticipos
                let total_anticipos_gravados = &self.get_anticipos().iter()
                    .filter(|e| {
                        if let Some(tipo) = e.tipo {
                            if let Some(catalog53) = catalog53_value_of_code(tipo) {
                                catalog53 == Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .map(|e| e.monto)
                    .fold(dec!(0), |a, b| a + b);

                // Descuentos
                let descuentos =
                    &self
                        .get_descuentos()
                        .iter()
                        .fold(HashMap::new(), |mut acc, current| {
                            if let Some(tipo) = current.tipo {
                                if let Some(catalog53) = catalog53_value_of_code(tipo) {
                                    let monto =
                                        acc.get(&catalog53).unwrap_or(&dec!(0)) + current.monto;
                                    acc.insert(catalog53, monto);
                                }
                            }
                            acc
                        });

                let descuentos_que_afectan_base_imponible_sin_impuestos = if let Some(val) =
                    descuentos.get(&Catalog53::DescuentoGlobalAfectaBaseImponibleIgvIvap)
                {
                    *val
                } else {
                    dec!(0)
                };

                // Aplicar ANTICIPOS Y DESCUENTOS
                let gravado_base_imponible = gravado.base_imponible
                    - total_anticipos_gravados
                    - descuentos_que_afectan_base_imponible_sin_impuestos;

                let factor = if gravado.base_imponible > dec!(0) {
                    gravado_base_imponible / gravado.base_imponible
                } else {
                    dec!(1)
                };

                let total = (gravado.importe + ivap.importe + exportacion.importe) * factor;

                // Set final values
                let total_impuestos = TotalImpuestos {
                    ivap_importe: ivap.importe_igv,
                    ivap_base_imponible: ivap.base_imponible,

                    exportacion_importe: exportacion.importe_igv,
                    exportacion_base_imponible: exportacion.base_imponible,

                    isc_importe,
                    isc_base_imponible: *isc_base_imponible,
                    gravado_importe: gravado.importe_igv * factor,
                    gravado_base_imponible,

                    inafecto_importe: inafecto.importe_igv,
                    inafecto_base_imponible: inafecto.base_imponible,
                    exonerado_importe: exonerado.importe_igv,
                    exonerado_base_imponible: exonerado.base_imponible,
                    gratuito_importe: gratuito.importe_igv,
                    gratuito_base_imponible: gratuito.base_imponible,

                    icb_importe,

                    total,
                };

                self.set_totalimpuestos(total_impuestos);
                true
            }
        }
    }
}

struct Impuesto {
    pub base_imponible: Decimal,

    pub importe: Decimal,
    pub importe_igv: Decimal,
    pub importe_isc: Decimal,
    pub importe_icb: Decimal,
}

fn cal_impuesto_by_tipo(detalles: &[Detalle], categoria: Catalog5) -> Impuesto {
    let stream: Vec<&Detalle> = detalles
        .iter()
        .filter(|e| e.igv_tipo.is_some() && e.igv_tipo.is_some())
        .filter(|e| {
            if let Some(catalog7) = catalog7_value_of_code(e.igv_tipo.unwrap()) {
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
        .fold(dec!(0), |a, b| a + b.unwrap());
    let importe = stream
        .iter()
        .map(|e| e.total_impuestos)
        .filter(|e| e.is_some())
        .fold(dec!(0), |a, b| a + b.unwrap());

    let importe_isc = stream
        .iter()
        .map(|e| e.isc)
        .filter(|e| e.is_some())
        .fold(dec!(0), |a, b| a + b.unwrap());
    let importe_igv = stream
        .iter()
        .map(|e| e.igv)
        .filter(|e| e.is_some())
        .fold(dec!(0), |a, b| a + b.unwrap());
    let importe_icb = stream
        .iter()
        .map(|e| e.icb)
        .filter(|e| e.is_some())
        .fold(dec!(0), |a, b| a + b.unwrap());

    Impuesto {
        base_imponible,
        importe,
        importe_igv,
        importe_isc,
        importe_icb,
    }
}
