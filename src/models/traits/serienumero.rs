use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait SerieNumeroGetter {
    fn get_serienumero(&self) -> &str;
}

impl SerieNumeroGetter for Invoice {
    fn get_serienumero(&self) -> &str {
        self.serie_numero
    }
}

impl SerieNumeroGetter for CreditNote {
    fn get_serienumero(&self) -> &str {
        self.serie_numero
    }
}

impl SerieNumeroGetter for DebitNote {
    fn get_serienumero(&self) -> &str {
        self.serie_numero
    }
}
