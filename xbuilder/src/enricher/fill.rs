use crate::enricher::rules::phase1fill::despatch_advice::firmante::DespatchAdviceFirmanteFillRule;
use crate::enricher::rules::phase1fill::despatch_advice::peso_total::DespatchAdvicePesoTotalFillRule;
use crate::enricher::rules::phase1fill::despatch_advice::tipo_comprobante::DespatchAdviceTipoComprobanteFillRule;
use crate::enricher::rules::phase1fill::detalle::detalles::DetallesFillRule;
use crate::enricher::rules::phase1fill::fecha_emision::FechaEmisionFillRule;
use crate::enricher::rules::phase1fill::firmante::FirmanteFillRule;
use crate::enricher::rules::phase1fill::icb_tasa::IcbTasaFillRule;
use crate::enricher::rules::phase1fill::igv_tasa::IgvTasaFillRule;
use crate::enricher::rules::phase1fill::invoice::anticipos::InvoiceAnticiposFillRule;
use crate::enricher::rules::phase1fill::invoice::descuentos::InvoiceDescuentosFillRule;
use crate::enricher::rules::phase1fill::invoice::forma_de_pago::{
    InvoiceFormaDePagoFillRule, InvoiceFormaDePagoTipoRule, InvoiceFormaDePagoTotalRule,
};
use crate::enricher::rules::phase1fill::invoice::leyenda::{
    InvoiceLeyendaDetraccionFillRule, InvoiceLeyendaDireccionEntregaFillRule,
    InvoiceLeyendaPercepcionFillRule,
};
use crate::enricher::rules::phase1fill::invoice::tipo_comprobante::InvoiceTipoComprobanteFillRule;
use crate::enricher::rules::phase1fill::invoice::tipo_operacion::InvoiceTipoOperacionFillRule;
use crate::enricher::rules::phase1fill::ivap_tasa::IvapTasaFillRule;
use crate::enricher::rules::phase1fill::moneda::MonedaFillRule;
use crate::enricher::rules::phase1fill::note::creditnote::tipo_nota::CreditNoteTipoFillRule;
use crate::enricher::rules::phase1fill::note::debitnote::tipo_nota::DebitNoteTipoFillRule;
use crate::enricher::rules::phase1fill::note::tipo_comprobante_afectado::NoteComprobanteAfectadoTipoFillRule;
use crate::enricher::rules::phase1fill::perception::documento_id::PerceptionDocumentoIdFillRule;
use crate::enricher::rules::phase1fill::perception::moneda::PerceptionMonedaFillRule;
use crate::enricher::rules::phase1fill::proveedor::ProveedorFillRule;
use crate::enricher::rules::phase1fill::retention::documento_id::RetentionDocumentoIdFillRule;
use crate::enricher::rules::phase1fill::retention::moneda::RetentionMonedaFillRule;
use crate::enricher::rules::phase1fill::summary_documents::documento_id::SummaryDocumentsDocumentoIdFillRule;
use crate::enricher::rules::phase1fill::summary_documents::moneda::SummaryDocumentsMonedaFillRule;
use crate::enricher::rules::phase1fill::voided_documents::documento_id::VoidedDocumentsDocumentoIdFillRule;
use crate::enricher::rules::phase1fill::voided_documents::tipo_comprobante::VoidedDocumentsTipoComprobanteFillRule;
use crate::enricher::Defaults;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::VoidedDocuments;

pub trait Fill {
    fn fill(&mut self, defaults: &Defaults);
}

trait FillCommon {
    fn fill_common(&mut self, defaults: &Defaults);
}

trait FillInvoice {
    fn fill_invoice(&mut self, defaults: &Defaults);
}

trait FillCreditNote {
    fn fill_credit_note(&mut self, defaults: &Defaults);
}

trait FillDebitNote {
    fn fill_debit_note(&mut self, defaults: &Defaults);
}

trait FillVoidedDocuments {
    fn fill_voided_documents(&mut self, defaults: &Defaults);
}

trait FillSummaryDocuments {
    fn fill_summary_documents(&mut self, defaults: &Defaults);
}

trait FillDespatchAdvice {
    fn fill_despatch_advice(&mut self, defaults: &Defaults);
}

trait FillPerception {
    fn fill_perception(&mut self, defaults: &Defaults);
}

trait FillRetention {
    fn fill_retention(&mut self, defaults: &Defaults);
}

impl Fill for Invoice {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
        self.fill_invoice(defaults);
    }
}

impl Fill for CreditNote {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
        self.fill_credit_note(defaults);
    }
}

impl Fill for DebitNote {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
        self.fill_debit_note(defaults);
    }
}

impl Fill for VoidedDocuments {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_voided_documents(defaults);
    }
}

impl Fill for SummaryDocuments {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_summary_documents(defaults);
    }
}

impl Fill for Perception {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_perception(defaults);
    }
}

impl Fill for Retention {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_retention(defaults);
    }
}

impl Fill for DespatchAdvice {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_despatch_advice(defaults);
    }
}

impl<T> FillCommon for T
where
    T: DetallesFillRule
        + FechaEmisionFillRule
        + FirmanteFillRule
        + IcbTasaFillRule
        + IgvTasaFillRule
        + IvapTasaFillRule
        + MonedaFillRule
        + ProveedorFillRule,
{
    fn fill_common(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                FirmanteFillRule::fill(self).unwrap_or_default(),
                IcbTasaFillRule::fill(self, defaults).unwrap_or_default(),
                IgvTasaFillRule::fill(self, defaults).unwrap_or_default(),
                IvapTasaFillRule::fill(self, defaults).unwrap_or_default(),
                MonedaFillRule::fill(self).unwrap_or_default(),
                ProveedorFillRule::fill(self).unwrap_or_default(),
                DetallesFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillInvoice for T
where
    T: InvoiceAnticiposFillRule
        + InvoiceDescuentosFillRule
        + InvoiceFormaDePagoFillRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceFormaDePagoTipoRule
        + InvoiceLeyendaDetraccionFillRule
        + InvoiceLeyendaDireccionEntregaFillRule
        + InvoiceLeyendaPercepcionFillRule
        + InvoiceTipoComprobanteFillRule
        + InvoiceTipoOperacionFillRule,
{
    fn fill_invoice(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                InvoiceAnticiposFillRule::fill(self).unwrap_or_default(),
                InvoiceDescuentosFillRule::fill(self).unwrap_or_default(),
                InvoiceFormaDePagoFillRule::fill(self).unwrap_or_default(),
                InvoiceFormaDePagoTotalRule::fill(self).unwrap_or_default(),
                InvoiceFormaDePagoTipoRule::fill(self).unwrap_or_default(),
                InvoiceLeyendaDetraccionFillRule::fill(self).unwrap_or_default(),
                InvoiceLeyendaDireccionEntregaFillRule::fill(self).unwrap_or_default(),
                InvoiceLeyendaPercepcionFillRule::fill(self).unwrap_or_default(),
                InvoiceTipoComprobanteFillRule::fill(self).unwrap_or_default(),
                InvoiceTipoOperacionFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillCreditNote for T
where
    T: NoteComprobanteAfectadoTipoFillRule + CreditNoteTipoFillRule,
{
    fn fill_credit_note(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                NoteComprobanteAfectadoTipoFillRule::fill(self).unwrap_or_default(),
                CreditNoteTipoFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillDebitNote for T
where
    T: NoteComprobanteAfectadoTipoFillRule + DebitNoteTipoFillRule,
{
    fn fill_debit_note(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                NoteComprobanteAfectadoTipoFillRule::fill(self).unwrap_or_default(),
                DebitNoteTipoFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillVoidedDocuments for T
where
    T: FechaEmisionFillRule
        + FirmanteFillRule
        + VoidedDocumentsTipoComprobanteFillRule
        + VoidedDocumentsDocumentoIdFillRule,
{
    fn fill_voided_documents(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                FirmanteFillRule::fill(self).unwrap_or_default(),
                VoidedDocumentsTipoComprobanteFillRule::fill(self).unwrap_or_default(),
                VoidedDocumentsDocumentoIdFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillSummaryDocuments for T
where
    T: FechaEmisionFillRule
        + FirmanteFillRule
        + SummaryDocumentsMonedaFillRule
        + SummaryDocumentsDocumentoIdFillRule,
{
    fn fill_summary_documents(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                FirmanteFillRule::fill(self).unwrap_or_default(),
                SummaryDocumentsMonedaFillRule::fill(self).unwrap_or_default(),
                SummaryDocumentsDocumentoIdFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillPerception for T
where
    T: FechaEmisionFillRule
        + FirmanteFillRule
        + PerceptionMonedaFillRule
        + PerceptionDocumentoIdFillRule,
{
    fn fill_perception(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                FirmanteFillRule::fill(self).unwrap_or_default(),
                PerceptionMonedaFillRule::fill(self).unwrap_or_default(),
                PerceptionDocumentoIdFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillRetention for T
where
    T: FechaEmisionFillRule
        + FirmanteFillRule
        + RetentionMonedaFillRule
        + RetentionDocumentoIdFillRule,
{
    fn fill_retention(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                FirmanteFillRule::fill(self).unwrap_or_default(),
                RetentionMonedaFillRule::fill(self).unwrap_or_default(),
                RetentionDocumentoIdFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillDespatchAdvice for T
where
    T: FechaEmisionFillRule
        + DespatchAdviceFirmanteFillRule
        + DespatchAdviceTipoComprobanteFillRule
        + DespatchAdvicePesoTotalFillRule,
{
    fn fill_despatch_advice(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                FechaEmisionFillRule::fill(self, defaults).unwrap_or_default(),
                DespatchAdviceFirmanteFillRule::fill(self).unwrap_or_default(),
                DespatchAdviceTipoComprobanteFillRule::fill(self).unwrap_or_default(),
                DespatchAdvicePesoTotalFillRule::fill(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}
