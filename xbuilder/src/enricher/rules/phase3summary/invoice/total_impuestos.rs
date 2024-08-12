use std::collections::HashMap;

use anyhow::Result;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog5, Catalog53, FromCode};
use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::invoice::anticipos::InvoiceAnticiposGetter;
use crate::enricher::bounds::invoice::descuentos::InvoiceDescuentosGetter;
use crate::enricher::bounds::total_impuestos::{TotalImpuestosGetter, TotalImpuestosSetter};
use crate::enricher::rules::phase3summary::utils::cal_impuesto_by_tipo;
use crate::models::common::TotalImpuestos;

pub trait InvoiceTotalImpuestosSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> InvoiceTotalImpuestosSummaryRule for T
where
    T: TotalImpuestosGetter
        + TotalImpuestosSetter
        + DetallesGetter
        + InvoiceDescuentosGetter
        + InvoiceAnticiposGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.get_total_impuestos() {
            Some(..) => Ok(false),
            None => {
                let ivap = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::ImpuestoArrozPilado);
                let exportacion = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Exportacion);
                let gravado = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Igv);
                let inafecto = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Inafecto);
                let exonerado = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Exonerado);
                let gratuito = cal_impuesto_by_tipo(self.get_detalles(), Catalog5::Gratuito);

                // ICB
                let icb_importe = [
                    ivap.importe_icb,
                    exportacion.importe_icb,
                    gravado.importe_icb,
                    inafecto.importe_icb,
                    exonerado.importe_icb,
                    gratuito.importe_icb,
                ]
                .iter()
                .fold(Decimal::ZERO, |a, b| a + b);

                // ISC
                let isc_importe = [
                    ivap.importe_isc,
                    exportacion.importe_isc,
                    gravado.importe_isc,
                    inafecto.importe_isc,
                    exonerado.importe_isc,
                    gratuito.importe_isc,
                ]
                .iter()
                .fold(Decimal::ZERO, |a, b| a + b);

                let isc_base_imponible = &self
                    .get_detalles()
                    .iter()
                    .filter(|e| match (e.isc_tasa, e.isc) {
                        (Some(isc_tasa), Some(isc)) => {
                            isc_tasa > Decimal::ZERO && isc > Decimal::ZERO
                        }
                        _ => false,
                    })
                    .filter_map(|e| e.isc_base_imponible)
                    .fold(Decimal::ZERO, |a, b| a + b);

                // Anticipos
                let total_anticipos_gravados = &self.get_anticipos().iter()
                    .filter(|e| {
                        if let Some(tipo) = e.tipo {
                            if let Ok(catalog53) = Catalog53::from_code(tipo) {
                                catalog53 == Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .map(|e| e.monto)
                    .fold(Decimal::ZERO, |a, b| a + b);

                // Descuentos
                let descuentos =
                    &self
                        .get_descuentos()
                        .iter()
                        .fold(HashMap::new(), |mut acc, current| {
                            if let Some(tipo) = current.tipo {
                                if let Ok(catalog53) = Catalog53::from_code(tipo) {
                                    let monto = acc.get(&catalog53).unwrap_or(&Decimal::ZERO)
                                        + current.monto;
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
                    Decimal::ZERO
                };

                // Aplicar ANTICIPOS Y DESCUENTOS
                let gravado_base_imponible = gravado.base_imponible
                    - total_anticipos_gravados
                    - descuentos_que_afectan_base_imponible_sin_impuestos;

                let factor = if gravado.base_imponible > Decimal::ZERO {
                    gravado_base_imponible / gravado.base_imponible
                } else {
                    Decimal::ONE
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

                self.set_total_impuestos(total_impuestos);
                Ok(true)
            }
        }
    }
}
