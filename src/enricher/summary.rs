use crate::enricher::rules::phase3summary::invoice::detraccion::DetraccionSummaryRule;
use crate::enricher::rules::phase3summary::invoice::percepcion::PercepcionSummaryRule;
use crate::enricher::rules::phase3summary::invoice::total_importe::InvoiceTotalImporteSummaryRule;
use crate::enricher::rules::phase3summary::invoice::total_impuestos::InvoiceTotalImpuestosSummaryRule;
use crate::enricher::rules::phase3summary::leyenda::LeyendaIVAPSummaryRule;
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
    fn summary_creditnote(&mut self);
}

trait SummaryDebitNote {
    fn summary_debitnote(&mut self);
}

impl Summary for Invoice {
    fn summary(&mut self) {
        self.summary_common();
    }
}

impl Summary for CreditNote {
    fn summary(&mut self) {
        // self.summary_common();
    }
}

impl Summary for DebitNote {
    fn summary(&mut self) {
        // self.summary_common();
    }
}

impl<T> SummaryCommon for T
where
    T: InvoiceTotalImpuestosSummaryRule
        + InvoiceTotalImporteSummaryRule
        + LeyendaIVAPSummaryRule
        + PercepcionSummaryRule
        + DetraccionSummaryRule,
{
    fn summary_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = [
                InvoiceTotalImpuestosSummaryRule::summary(self),
                InvoiceTotalImporteSummaryRule::summary(self),
                LeyendaIVAPSummaryRule::summary(self),
                PercepcionSummaryRule::summary(self),
                DetraccionSummaryRule::summary(self),
            ];

            changed = results.contains(&true);
        }
    }
}
