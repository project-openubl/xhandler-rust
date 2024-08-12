use crate::enricher::rules::phase2process::detalle::detalles::DetallesProcessRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

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
//     fn process_creditnote(&mut self);
// }
//
// trait ProcessDebitNote {
//     fn process_debitnote(&mut self);
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
