use std::collections::HashMap;

use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{Catalog5, Catalog53, FromCode};
use crate::xbuilder::enricher::bounds::detalle::DetallesGetter;
use crate::xbuilder::enricher::bounds::invoice::anticipos::InvoiceAnticiposGetter;
use crate::xbuilder::enricher::bounds::invoice::cargos::InvoiceCargosGetter;
use crate::xbuilder::enricher::bounds::invoice::descuentos::InvoiceDescuentosGetter;
use crate::xbuilder::enricher::bounds::total_impuestos::{
    TotalImpuestosGetter, TotalImpuestosSetter,
};
use crate::xbuilder::enricher::rules::phase3_summary::impuesto::compute_impuesto_by_tipo;
use crate::xbuilder::models::common::TotalImpuestos;

pub trait InvoiceTotalImpuestosSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> InvoiceTotalImpuestosSummaryRule for T
where
    T: TotalImpuestosGetter
        + TotalImpuestosSetter
        + DetallesGetter
        + InvoiceDescuentosGetter
        + InvoiceCargosGetter
        + InvoiceAnticiposGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.total_impuestos() {
            Some(..) => Ok(false),
            None => {
                let ivap = compute_impuesto_by_tipo(self.detalles(), Catalog5::ImpuestoArrozPilado);
                let exportacion = compute_impuesto_by_tipo(self.detalles(), Catalog5::Exportacion);
                let gravado = compute_impuesto_by_tipo(self.detalles(), Catalog5::Igv);
                let inafecto = compute_impuesto_by_tipo(self.detalles(), Catalog5::Inafecto);
                let exonerado = compute_impuesto_by_tipo(self.detalles(), Catalog5::Exonerado);
                let gratuito = compute_impuesto_by_tipo(self.detalles(), Catalog5::Gratuito);

                // ICB
                let icb_importe = [
                    ivap.icb_importe,
                    exportacion.icb_importe,
                    gravado.icb_importe,
                    inafecto.icb_importe,
                    exonerado.icb_importe,
                    gratuito.icb_importe,
                ]
                .iter()
                .fold(Decimal::ZERO, |a, b| a + b);

                // ISC
                let isc_importe = [
                    ivap.isc_importe,
                    exportacion.isc_importe,
                    gravado.isc_importe,
                    inafecto.isc_importe,
                    exonerado.isc_importe,
                    gratuito.isc_importe,
                ]
                .iter()
                .fold(Decimal::ZERO, |a, b| a + b);

                let isc_base_imponible = &self
                    .detalles()
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
                let total_anticipos_gravados = &self.anticipos().iter()
                    .filter(|e| {
                        if let Some(tipo) = e.tipo.as_deref() {
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
                        .descuentos()
                        .iter()
                        .fold(HashMap::new(), |mut acc, current| {
                            if let Some(tipo) = current.tipo.as_deref() {
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

                let cargos_que_afectan_base_imponible_sin_impuestos: Decimal = self
                    .cargos()
                    .iter()
                    .filter(|cargo| cargo.afecta_base_imponible())
                    .map(|cargo| cargo.monto)
                    .sum();

                // Aplicar ANTICIPOS, DESCUENTOS Y CARGOS
                let gravado_base_imponible = gravado.base_imponible
                    - total_anticipos_gravados
                    - descuentos_que_afectan_base_imponible_sin_impuestos
                    + cargos_que_afectan_base_imponible_sin_impuestos;

                let factor = if gravado.base_imponible > Decimal::ZERO {
                    gravado_base_imponible / gravado.base_imponible
                } else {
                    Decimal::ONE
                };

                let total = (gravado.importe + ivap.importe + exportacion.importe) * factor;

                // Set final values
                let total_impuestos = TotalImpuestos {
                    ivap_importe: ivap.igv_importe,
                    ivap_base_imponible: ivap.base_imponible,

                    exportacion_importe: exportacion.igv_importe,
                    exportacion_base_imponible: exportacion.base_imponible,

                    isc_importe,
                    isc_base_imponible: *isc_base_imponible,
                    gravado_importe: gravado.igv_importe * factor,
                    gravado_base_imponible,

                    inafecto_importe: inafecto.igv_importe,
                    inafecto_base_imponible: inafecto.base_imponible,
                    exonerado_importe: exonerado.igv_importe,
                    exonerado_base_imponible: exonerado.base_imponible,
                    gratuito_importe: gratuito.igv_importe,
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
