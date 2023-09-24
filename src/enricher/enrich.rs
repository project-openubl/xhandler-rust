use chrono::NaiveDate;

use crate::enricher::rules::phase1enrich::detalle::detalles::DetallesEnrichRule;
use crate::enricher::rules::phase1enrich::fecha_emision::FechaEmisionEnrichRule;
use crate::enricher::rules::phase1enrich::firmante::FirmanteEnrichRule;
use crate::enricher::rules::phase1enrich::icbtasa::ICBTasaEnrichRule;
use crate::enricher::rules::phase1enrich::igvtasa::IGVTasaEnrichRule;
use crate::enricher::rules::phase1enrich::invoice::anticipos::InvoiceAnticiposEnrichRule;
use crate::enricher::rules::phase1enrich::invoice::descuentos::InvoiceDescuentosEnrichRule;
use crate::enricher::rules::phase1enrich::invoice::formadepago::{
    InvoiceFormaDePagoEnrichRule, InvoiceFormaDePagoTotalRule,
};
use crate::enricher::rules::phase1enrich::invoice::leyenda::{
    InvoiceLeyendaDetraccionEnrichRule, InvoiceLeyendaDireccionEntregaEnrichRule,
    InvoiceLeyendaPercepcionEnrichRule,
};
use crate::enricher::rules::phase1enrich::invoice::tipocomprobante::InvoiceTipoComprobanteEnrichRule;
use crate::enricher::rules::phase1enrich::invoice::tipooperacion::InvoiceTipoOperacionEnrichRule;
use crate::enricher::rules::phase1enrich::ivaptasa::IVAPTasaEnrichRule;
use crate::enricher::rules::phase1enrich::moneda::MonedaEnrichRule;
use crate::enricher::rules::phase1enrich::note::creditnote::tiponota::CreditNoteTipoEnrichRule;
use crate::enricher::rules::phase1enrich::note::debitnote::tiponota::DebitNoteTipoEnrichRule;
use crate::enricher::rules::phase1enrich::note::tipocomprobanteafectado::NoteTipoComprobanteAfectadoEnrichRule;
use crate::enricher::rules::phase1enrich::proveedor::ProveedorEnrichRule;
use crate::enricher::rules::phase2process::detalle::detalles::DetallesProcessRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub struct Defaults {
    pub icb_tasa: f32,
    pub igv_tasa: f32,
    pub ivap_tasa: f32,

    pub date: NaiveDate,
}

//

pub trait EnrichTrait {
    fn enrich(&mut self, defaults: &Defaults);
}

trait EnrichCommonTrait {
    fn enrich_common(&mut self, defaults: &Defaults);
}

trait EnrichInvoiceTrait {
    fn enrich_invoice(&mut self, defaults: &Defaults);
}

trait EnrichCreditNoteTrait {
    fn enrich_creditnote(&mut self, defaults: &Defaults);
}

trait EnrichDebitNoteTrait {
    fn enrich_debitnote(&mut self, defaults: &Defaults);
}

//

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

// Enrich implementations

impl EnrichTrait for Invoice {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_invoice(defaults);

        self.process_common();
    }
}

impl EnrichTrait for CreditNote {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_creditnote(defaults);

        self.process_common();
    }
}

impl EnrichTrait for DebitNote {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_debitnote(defaults);

        self.process_common();
    }
}

impl<T> EnrichCommonTrait for T
where
    T: DetallesEnrichRule
        + FechaEmisionEnrichRule
        + FirmanteEnrichRule
        + ICBTasaEnrichRule
        + IGVTasaEnrichRule
        + IVAPTasaEnrichRule
        + MonedaEnrichRule
        + ProveedorEnrichRule,
{
    fn enrich_common(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                FechaEmisionEnrichRule::enrich(self, defaults),
                FirmanteEnrichRule::enrich(self),
                ICBTasaEnrichRule::enrich(self, defaults),
                IGVTasaEnrichRule::enrich(self, defaults),
                IVAPTasaEnrichRule::enrich(self, defaults),
                MonedaEnrichRule::enrich(self),
                ProveedorEnrichRule::enrich(self),
                DetallesEnrichRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichInvoiceTrait for T
where
    T: InvoiceAnticiposEnrichRule
        + InvoiceDescuentosEnrichRule
        + InvoiceFormaDePagoEnrichRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceLeyendaDetraccionEnrichRule
        + InvoiceLeyendaDireccionEntregaEnrichRule
        + InvoiceLeyendaPercepcionEnrichRule
        + InvoiceFormaDePagoEnrichRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceTipoComprobanteEnrichRule
        + InvoiceTipoOperacionEnrichRule,
{
    fn enrich_invoice(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                InvoiceAnticiposEnrichRule::enrich(self),
                InvoiceDescuentosEnrichRule::enrich(self),
                InvoiceFormaDePagoEnrichRule::enrich(self),
                InvoiceFormaDePagoTotalRule::enrich(self),
                InvoiceLeyendaDetraccionEnrichRule::enrich(self),
                InvoiceLeyendaDireccionEntregaEnrichRule::enrich(self),
                InvoiceLeyendaPercepcionEnrichRule::enrich(self),
                InvoiceTipoComprobanteEnrichRule::enrich(self),
                InvoiceTipoOperacionEnrichRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichCreditNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoEnrichRule + CreditNoteTipoEnrichRule,
{
    fn enrich_creditnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                NoteTipoComprobanteAfectadoEnrichRule::enrich(self),
                CreditNoteTipoEnrichRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichDebitNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoEnrichRule + DebitNoteTipoEnrichRule,
{
    fn enrich_debitnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                NoteTipoComprobanteAfectadoEnrichRule::enrich(self),
                DebitNoteTipoEnrichRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

// Process implementations

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
            let results = vec![DetallesProcessRule::process(self)];

            changed = results.contains(&true);
        }
    }
}
