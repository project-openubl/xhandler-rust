use crate::enricher::rules::phase2process::detalle::detalles::DetallesProcessRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait ProcessTrait {
    fn process(&mut self);
}

trait ProcessCommonTrait {
    fn process_common(&mut self);
}

trait ProcessInvoiceTrait {
    fn process_invoice(&mut self);
}

trait ProcessCreditNoteTrait {
    fn process_creditnote(&mut self);
}

trait ProcessDebitNoteTrait {
    fn process_debitnote(&mut self);
}

impl ProcessTrait for Invoice {
    fn process(&mut self) {
        self.process_common();
    }
}

impl ProcessTrait for CreditNote {
    fn process(&mut self) {
        self.process_common();
    }
}

impl ProcessTrait for DebitNote {
    fn process(&mut self) {
        self.process_common();
    }
}

impl<T> ProcessCommonTrait for T
where
    T: DetallesProcessRule,
{
    fn process_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = [DetallesProcessRule::process(self)];

            changed = results.contains(&true);
        }
    }
}
