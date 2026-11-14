use crate::xbuilder::enricher::rules::phase3_summary::invoice::detraccion::InvoiceDetraccionSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::invoice::percepcion::InvoicePercepcionSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::invoice::total_importe::InvoiceTotalImporteSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::invoice::total_impuestos::InvoiceTotalImpuestosSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::leyenda::LeyendaIvapSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::note::total_importe::NoteTotalImporteSummaryRule;
use crate::xbuilder::enricher::rules::phase3_summary::note::total_impuestos::NoteTotalImpuestosSummaryRule;
use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::despatch_advice::DespatchAdvice;
use crate::xbuilder::models::invoice::Invoice;
use crate::xbuilder::models::perception_retention::{Perception, Retention};
use crate::xbuilder::models::summary_documents::SummaryDocuments;
use crate::xbuilder::models::voided_documents::VoidedDocuments;

pub trait Summary {
    fn summary(&mut self);
}

trait SummaryCommon {
    fn summary_common(&mut self);
}

trait SummaryInvoice {
    fn summary_invoice(&mut self);
}

trait SummaryCreditNote {
    fn summary_credit_note(&mut self);
}

trait SummaryDebitNote {
    fn summary_debit_note(&mut self);
}

impl Summary for Invoice {
    fn summary(&mut self) {
        self.summary_common();
        self.summary_invoice();
    }
}

impl Summary for CreditNote {
    fn summary(&mut self) {
        self.summary_common();
        self.summary_credit_note()
    }
}

impl Summary for DebitNote {
    fn summary(&mut self) {
        self.summary_common();
        self.summary_debit_note();
    }
}

impl Summary for VoidedDocuments {
    fn summary(&mut self) {}
}

impl Summary for SummaryDocuments {
    fn summary(&mut self) {}
}

impl Summary for Perception {
    fn summary(&mut self) {}
}

impl Summary for Retention {
    fn summary(&mut self) {}
}

impl Summary for DespatchAdvice {
    fn summary(&mut self) {}
}

impl<T> SummaryCommon for T
where
    T: LeyendaIvapSummaryRule,
{
    fn summary_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = [LeyendaIvapSummaryRule::summary(self).unwrap_or_default()];

            changed = results.contains(&true);
        }
    }
}

impl<T> SummaryInvoice for T
where
    T: InvoiceTotalImpuestosSummaryRule
        + InvoiceTotalImporteSummaryRule
        + InvoicePercepcionSummaryRule
        + InvoiceDetraccionSummaryRule,
{
    fn summary_invoice(&mut self) {
        let mut changed = true;

        while changed {
            let results = [
                InvoiceTotalImpuestosSummaryRule::summary(self).unwrap_or_default(),
                InvoiceTotalImporteSummaryRule::summary(self).unwrap_or_default(),
                InvoicePercepcionSummaryRule::summary(self).unwrap_or_default(),
                InvoiceDetraccionSummaryRule::summary(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> SummaryCreditNote for T
where
    T: NoteTotalImpuestosSummaryRule + NoteTotalImporteSummaryRule,
{
    fn summary_credit_note(&mut self) {
        let mut changed = true;

        while changed {
            let results = [
                NoteTotalImpuestosSummaryRule::summary(self).unwrap_or_default(),
                NoteTotalImporteSummaryRule::summary(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> SummaryDebitNote for T
where
    T: NoteTotalImpuestosSummaryRule + NoteTotalImporteSummaryRule,
{
    fn summary_debit_note(&mut self) {
        let mut changed = true;

        while changed {
            let results = [
                NoteTotalImpuestosSummaryRule::summary(self).unwrap_or_default(),
                NoteTotalImporteSummaryRule::summary(self).unwrap_or_default(),
            ];

            changed = results.contains(&true);
        }
    }
}
