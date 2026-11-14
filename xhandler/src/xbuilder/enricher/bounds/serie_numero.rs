use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::despatch_advice::DespatchAdvice;
use crate::xbuilder::models::invoice::Invoice;

pub trait SerieNumeroGetter {
    fn serie_numero(&self) -> &str;
}

impl SerieNumeroGetter for Invoice {
    fn serie_numero(&self) -> &str {
        &self.serie_numero
    }
}

impl SerieNumeroGetter for CreditNote {
    fn serie_numero(&self) -> &str {
        &self.serie_numero
    }
}

impl SerieNumeroGetter for DebitNote {
    fn serie_numero(&self) -> &str {
        &self.serie_numero
    }
}

impl SerieNumeroGetter for DespatchAdvice {
    fn serie_numero(&self) -> &str {
        &self.serie_numero
    }
}
