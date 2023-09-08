use chrono::NaiveDate;
use crate::content::models::invoice::Invoice;
use crate::enricher::rules::fecha_emision::FechaEmisionRule;
use crate::enricher::rules::firmante::FirmanteRule;
use crate::enricher::rules::moneda::{MonedaRule};
use crate::enricher::rules::proveedor::ProveedorRule;

pub struct Defaults {
    pub icb_tasa: usize,
    pub igv_tasa: usize,
    pub ivap_tasa: usize,

    pub date: NaiveDate,
}

pub fn enrich(invoice: &mut Invoice, defaults: Defaults) {
    let mut changed = true;

    while changed {
        let mut results = vec![];

        results.push(invoice.enrich_moneda());
        results.push(invoice.enrich_fechaemision(&defaults));
        results.push(invoice.enrich_proveedor());
        results.push(invoice.enrich_firmante());

        changed = results.contains(&true);
    }
}