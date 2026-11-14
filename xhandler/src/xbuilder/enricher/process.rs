use crate::xbuilder::enricher::rules::phase2_process::detalle::detalles::DetallesProcessRule;
use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::despatch_advice::DespatchAdvice;
use crate::xbuilder::models::invoice::Invoice;
use crate::xbuilder::models::perception_retention::{Perception, Retention};
use crate::xbuilder::models::summary_documents::SummaryDocuments;
use crate::xbuilder::models::voided_documents::VoidedDocuments;

pub trait Process {
    fn process(&mut self);
}

trait ProcessCommon {
    fn process_common(&mut self);
}

// trait ProcessInvoice {
//     fn process_invoice(&mut self);
// }
//
// trait ProcessCreditNote {
//     fn process_credit_note(&mut self);
// }
//
// trait ProcessDebitNote {
//     fn process_debit_note(&mut self);
// }

impl Process for Invoice {
    fn process(&mut self) {
        self.process_common();
    }
}

impl Process for CreditNote {
    fn process(&mut self) {
        self.process_common();
    }
}

impl Process for DebitNote {
    fn process(&mut self) {
        self.process_common();
    }
}

impl Process for VoidedDocuments {
    fn process(&mut self) {}
}

impl Process for SummaryDocuments {
    fn process(&mut self) {}
}

impl Process for Perception {
    fn process(&mut self) {}
}

impl Process for Retention {
    fn process(&mut self) {}
}

impl Process for DespatchAdvice {
    fn process(&mut self) {}
}

impl<T> ProcessCommon for T
where
    T: DetallesProcessRule,
{
    fn process_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = [DetallesProcessRule::process(self).unwrap_or_default()];

            changed = results.contains(&true);
        }
    }
}
