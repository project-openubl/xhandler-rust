use anyhow::{anyhow, Result};

use crate::catalogs::{
    Catalog10, Catalog12, Catalog19, Catalog53, Catalog6, Catalog7, Catalog8, Catalog9, FromCode,
};
use crate::models::common::{Cliente, Detalle};
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::VoidedDocuments;

/// Checks, after enrichment, the catalog codes xhandler-java rejects and the
/// required values that have no default
pub trait Validate {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Fails when `value` is given and is not a code (or name) of the catalog
fn check<C: FromCode>(field: &str, value: Option<&str>) -> Result<()> {
    match value {
        Some(value) if !value.is_empty() => C::from_code(value)
            .map(|_| ())
            .map_err(|error| anyhow!("{field}: {error}")),
        _ => Ok(()),
    }
}

fn check_sales_document(cliente: &Cliente, detalles: &[Detalle]) -> Result<()> {
    check::<Catalog6>(
        "cliente.tipo_documento_identidad",
        Some(&cliente.tipo_documento_identidad),
    )?;
    for (index, detalle) in detalles.iter().enumerate() {
        check::<Catalog7>(
            &format!("detalles[{index}].igv_tipo"),
            detalle.igv_tipo.as_deref(),
        )?;
        check::<Catalog8>(
            &format!("detalles[{index}].isc_tipo"),
            detalle.isc_tipo.as_deref(),
        )?;
    }
    Ok(())
}

impl Validate for Invoice {
    fn validate(&self) -> Result<()> {
        check_sales_document(&self.cliente, &self.detalles)?;
        for (index, descuento) in self.descuentos.iter().enumerate() {
            check::<Catalog53>(
                &format!("descuentos[{index}].tipo"),
                descuento.tipo.as_deref(),
            )?;
        }
        for (index, anticipo) in self.anticipos.iter().enumerate() {
            check::<Catalog53>(
                &format!("anticipos[{index}].tipo"),
                anticipo.tipo.as_deref(),
            )?;
            check::<Catalog12>(
                &format!("anticipos[{index}].comprobante_tipo"),
                anticipo.comprobante_tipo.as_deref(),
            )?;
        }
        for (index, cargo) in self.cargos.iter().enumerate() {
            check::<Catalog53>(&format!("cargos[{index}].tipo"), Some(&cargo.tipo))?;
        }
        Ok(())
    }
}

impl Validate for CreditNote {
    fn validate(&self) -> Result<()> {
        check_sales_document(&self.cliente, &self.detalles)?;
        check::<Catalog9>("tipo_nota", self.tipo_nota.as_deref())
    }
}

impl Validate for DebitNote {
    fn validate(&self) -> Result<()> {
        check_sales_document(&self.cliente, &self.detalles)?;
        check::<Catalog10>("tipo_nota", self.tipo_nota.as_deref())
    }
}

impl Validate for SummaryDocuments {
    fn validate(&self) -> Result<()> {
        for (index, item) in self.comprobantes.iter().enumerate() {
            check::<Catalog19>(
                &format!("comprobantes[{index}].tipo_operacion"),
                Some(&item.tipo_operacion),
            )?;
        }
        Ok(())
    }
}

// Nothing to check for these documents
impl Validate for VoidedDocuments {}
impl Validate for Perception {}
impl Validate for Retention {}
impl Validate for DespatchAdvice {
    fn validate(&self) -> Result<()> {
        if self.envio.fecha_traslado.is_none() {
            return Err(anyhow!("envio.fecha_traslado is required"));
        }
        Ok(())
    }
}
