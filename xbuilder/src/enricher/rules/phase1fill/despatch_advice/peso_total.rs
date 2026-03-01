use anyhow::Result;

use crate::enricher::bounds::despatch_advice::{
    DespatchAdvicePesoTotalFormattedGetter, DespatchAdvicePesoTotalFormattedSetter,
    DespatchAdvicePesoTotalGetter,
};

pub trait DespatchAdvicePesoTotalFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DespatchAdvicePesoTotalFillRule for T
where
    T: DespatchAdvicePesoTotalFormattedGetter
        + DespatchAdvicePesoTotalFormattedSetter
        + DespatchAdvicePesoTotalGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_peso_total_formatted() {
            Some(_) => Ok(false),
            None => {
                let peso = self.get_peso_total().round_dp(3);
                self.set_peso_total_formatted(format!("{:.3}", peso));
                Ok(true)
            }
        }
    }
}
