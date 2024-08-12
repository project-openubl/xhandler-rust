use crate::enricher::rules::phase3summary::invoice::detraccion::DetraccionSummaryRule;
use crate::enricher::rules::phase3summary::invoice::percepcion::PercepcionSummaryRule;
use crate::enricher::rules::phase3summary::invoice::total_importe::InvoiceTotalImporteSummaryRule;
use crate::enricher::rules::phase3summary::invoice::total_impuestos::InvoiceTotalImpuestosSummaryRule;
use crate::enricher::rules::phase3summary::leyenda::LeyendaIVAPSummaryRule;
use crate::enricher::rules::phase3summary::note::total_importe::NoteTotalImporteSummaryRule;
use crate::enricher::rules::phase3summary::note::total_impuestos::NoteTotalImpuestosSummaryRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

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

impl<T> SummaryCommon for T
where
    T: LeyendaIVAPSummaryRule,
{
    fn summary_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = [LeyendaIVAPSummaryRule::summary(self).unwrap_or_default()];

            changed = results.contains(&true);
        }
    }
}

impl<T> SummaryInvoice for T
where
    T: InvoiceTotalImpuestosSummaryRule
        + InvoiceTotalImporteSummaryRule
        + PercepcionSummaryRule
        + DetraccionSummaryRule,
{
    fn summary_invoice(&mut self) {
        let mut changed = true;

        while changed {
            let results = [
                InvoiceTotalImpuestosSummaryRule::summary(self).unwrap_or_default(),
                InvoiceTotalImporteSummaryRule::summary(self).unwrap_or_default(),
                PercepcionSummaryRule::summary(self).unwrap_or_default(),
                DetraccionSummaryRule::summary(self).unwrap_or_default(),
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
