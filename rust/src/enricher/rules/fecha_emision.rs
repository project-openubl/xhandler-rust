use chrono::NaiveDate;
use crate::content::models::invoice::Invoice;
use crate::enricher::enricher::Defaults;

pub trait FechaEmisionRule {
    fn enrich_fechaemision(&mut self, defaults: &Defaults) -> bool;
}

pub trait FechaEmision {
    fn get_fechaemision(&self) -> &Option<NaiveDate>;
    fn set_fechaemision(&mut self, val: Option<NaiveDate>);
}

impl<T> FechaEmisionRule for T
    where T: FechaEmision, {
    fn enrich_fechaemision(&mut self, defaults: &Defaults) -> bool {
        match &self.get_fechaemision() {
            Some(..) => false,
            None => {
                self.set_fechaemision(Some(defaults.date));
                true
            }
        }
    }
}

impl FechaEmision for Invoice {
    fn get_fechaemision(&self) -> &Option<NaiveDate> {
        &self.fecha_emision
    }

    fn set_fechaemision(&mut self, val: Option<NaiveDate>) {
        self.fecha_emision = val;
    }
}