use std::collections::HashMap;

use anyhow::Result;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog5, Catalog53, Catalog7, FromCode};
use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::bounds::igv::IgvTasaGetter;
use crate::enricher::bounds::invoice::anticipos::InvoiceAnticiposGetter;
use crate::enricher::bounds::invoice::descuentos::InvoiceDescuentosGetter;
use crate::enricher::bounds::invoice::total_importe::{
    InvoiceTotalImporteGetter, InvoiceTotalImporteSetter,
};
use crate::models::common::TotalImporteInvoice;

pub trait InvoiceTotalImporteSummaryRule {
    fn summary(&mut self) -> Result<bool>;
}

impl<T> InvoiceTotalImporteSummaryRule for T
where
    T: InvoiceTotalImporteGetter
        + InvoiceTotalImporteSetter
        + DetallesGetter
        + InvoiceDescuentosGetter
        + InvoiceAnticiposGetter
        + IgvTasaGetter,
{
    fn summary(&mut self) -> Result<bool> {
        match &self.get_total_importe() {
            Some(..) => Ok(false),
            None => {
                let total_impuestos = self
                    .get_detalles()
                    .iter()
                    .filter_map(|e| e.total_impuestos)
                    .fold(Decimal::ZERO, |a, b| a + b);

                let importe_sin_impuestos = self
                    .get_detalles()
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
                    .fold(Decimal::ZERO, |a, b| a + b);

                let importe_con_impuestos = importe_sin_impuestos + total_impuestos;

                // Anticipos
                let anticipos = self
                    .get_anticipos()
                    .iter()
                    .map(|e| e.monto)
                    .fold(Decimal::ZERO, |a, b| a + b);

                let importe_total = importe_con_impuestos - anticipos;

                // Descuentos
                let descuentos =
                    self.get_descuentos()
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
                self.set_total_importe(TotalImporteInvoice {
                    importe_sin_impuestos,
                    importe_con_impuestos,
                    descuentos: descuentos_que_no_afectan_base_imponible_sin_impuestos,
                    anticipos,
                    importe: importe_total,
                });
                Ok(true)
            }
        }
    }
}
