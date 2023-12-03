use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::catalogs::{Catalog5, Catalog53, catalog53_value_of_code, catalog7_value_of_code};
use crate::models::general::TotalImporte;
use crate::models::traits::detalle::DetallesGetter;
use crate::models::traits::igv::IGVTasaGetter;
use crate::models::traits::invoice::anticipos::InvoiceAnticiposGetter;
use crate::models::traits::invoice::descuentos::InvoiceDescuentosGetter;
use crate::models::traits::totalimporte::{TotalImporteGetter, TotalImporteSetter};

pub trait InvoiceTotalImporteSummaryRule {
    fn summary(&mut self) -> bool;
}

impl<T> InvoiceTotalImporteSummaryRule for T
where
    T: TotalImporteGetter
        + TotalImporteSetter
        + DetallesGetter
        + InvoiceDescuentosGetter
        + InvoiceAnticiposGetter
        + IGVTasaGetter,
{
    fn summary(&mut self) -> bool {
        match &self.get_totalimporte() {
            Some(..) => false,
            None => {
                let total_impuestos = self
                    .get_detalles()
                    .iter()
                    .filter_map(|e| e.total_impuestos)
                    .fold(dec!(0), |a, b| a + b);

                let importe_sin_impuestos = self
                    .get_detalles()
                    .iter()
                    .filter(|e| {
                        if let Some(catalog7) = catalog7_value_of_code(e.igv_tipo.unwrap_or("")) {
                            catalog7.tax_category() != Catalog5::Gratuito
                        } else {
                            false
                        }
                    })
                    .filter_map(|detalle| {
                        if detalle
                            .isc_base_imponible
                            .is_some_and(|base_imponible| base_imponible > dec!(0))
                        {
                            detalle.isc_base_imponible
                        } else {
                            detalle.igv_base_imponible
                        }
                    })
                    .fold(dec!(0), |a, b| a + b);

                let importe_con_impuestos = importe_sin_impuestos + total_impuestos;

                // Anticipos
                let anticipos = self
                    .get_anticipos()
                    .iter()
                    .map(|e| e.monto)
                    .fold(dec!(0), |a, b| a + b);

                let importe_total = importe_con_impuestos - anticipos;

                // Descuentos
                let descuentos =
                    self.get_descuentos()
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
                    Decimal::ZERO
                };
                let descuentos_que_afectan_base_imponible_con_impuestos =
                    descuentos_que_afectan_base_imponible_sin_impuestos
                        * (self.get_igv_tasa().unwrap_or(Decimal::ZERO) + Decimal::ONE);
                let descuentos_que_no_afectan_base_imponible_sin_impuestos = if let Some(val) =
                    descuentos.get(&Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap)
                {
                    *val
                } else {
                    Decimal::ZERO
                };

                //
                let importe_sin_impuestos =
                    importe_sin_impuestos - descuentos_que_afectan_base_imponible_sin_impuestos;
                let importe_con_impuestos =
                    importe_con_impuestos - descuentos_que_afectan_base_imponible_con_impuestos;
                let importe_total = importe_total
                    - descuentos_que_afectan_base_imponible_con_impuestos
                    - descuentos_que_no_afectan_base_imponible_sin_impuestos;

                //
                self.set_totalimporte(TotalImporte {
                    importe_sin_impuestos,
                    importe_con_impuestos,
                    descuentos: descuentos_que_no_afectan_base_imponible_sin_impuestos,
                    anticipos,
                    importe: importe_total,
                });
                true
            }
        }
    }
}
