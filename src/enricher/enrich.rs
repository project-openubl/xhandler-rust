use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::enricher::rules::phase1fill::detalle::detalles::DetallesEnrichRule;
use crate::enricher::rules::phase1fill::fecha_emision::FechaEmisionEnrichRule;
use crate::enricher::rules::phase1fill::firmante::FirmanteEnrichRule;
use crate::enricher::rules::phase1fill::icbtasa::ICBTasaEnrichRule;
use crate::enricher::rules::phase1fill::igvtasa::IGVTasaEnrichRule;
use crate::enricher::rules::phase1fill::invoice::anticipos::InvoiceAnticiposEnrichRule;
use crate::enricher::rules::phase1fill::invoice::descuentos::InvoiceDescuentosEnrichRule;
use crate::enricher::rules::phase1fill::invoice::formadepago::{
    InvoiceFormaDePagoEnrichRule, InvoiceFormaDePagoTotalRule,
};
use crate::enricher::rules::phase1fill::invoice::leyenda::{
    InvoiceLeyendaDetraccionEnrichRule, InvoiceLeyendaDireccionEntregaEnrichRule,
    InvoiceLeyendaPercepcionEnrichRule,
};
use crate::enricher::rules::phase1fill::invoice::tipocomprobante::InvoiceTipoComprobanteEnrichRule;
use crate::enricher::rules::phase1fill::invoice::tipooperacion::InvoiceTipoOperacionEnrichRule;
use crate::enricher::rules::phase1fill::ivaptasa::IVAPTasaEnrichRule;
use crate::enricher::rules::phase1fill::moneda::MonedaEnrichRule;
use crate::enricher::rules::phase1fill::note::creditnote::tiponota::CreditNoteTipoEnrichRule;
use crate::enricher::rules::phase1fill::note::debitnote::tiponota::DebitNoteTipoEnrichRule;
use crate::enricher::rules::phase1fill::note::tipocomprobanteafectado::NoteTipoComprobanteAfectadoEnrichRule;
use crate::enricher::rules::phase1fill::proveedor::ProveedorEnrichRule;
use crate::enricher::rules::phase2process::detalle::detalles::DetallesProcessRule;
use crate::enricher::rules::phase3summary::invoice::percepcion::PercepcionSummaryRule;
use crate::enricher::rules::phase3summary::invoice::totalimporte::InvoiceTotalImporteSummaryRule;
use crate::enricher::rules::phase3summary::invoice::totalimpuestos::InvoiceTotalImpuestosSummaryRule;
use crate::enricher::rules::phase3summary::leyenda::LeyendaIVAPSummaryRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub struct Defaults {
    pub icb_tasa: Decimal,
    pub igv_tasa: Decimal,
    pub ivap_tasa: Decimal,

    pub date: NaiveDate,
}

// ENRICH

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

// PROCESS

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

// SUMMARY

pub trait SummaryTrait {
    fn summary(&mut self);
}

trait SummaryCommonTrait {
    fn summary_common(&mut self);
}

trait SummaryInvoiceTrait {
    fn summary_invoice(&mut self);
}

trait SummaryCreditNoteTrait {
    fn summary_creditnote(&mut self);
}

trait SummaryDebitNoteTrait {
    fn summary_debitnote(&mut self);
}

// Enrich implementations

impl EnrichTrait for Invoice {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_invoice(defaults);

        self.process_common();

        self.summary_common();
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
                FechaEmisionEnrichRule::fill(self, defaults),
                FirmanteEnrichRule::fill(self),
                ICBTasaEnrichRule::fill(self, defaults),
                IGVTasaEnrichRule::fill(self, defaults),
                IVAPTasaEnrichRule::fill(self, defaults),
                MonedaEnrichRule::fill(self),
                ProveedorEnrichRule::fill(self),
                DetallesEnrichRule::fill(self),
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
                InvoiceAnticiposEnrichRule::fill(self),
                InvoiceDescuentosEnrichRule::fill(self),
                InvoiceFormaDePagoEnrichRule::fill(self),
                InvoiceFormaDePagoTotalRule::fill(self),
                InvoiceLeyendaDetraccionEnrichRule::fill(self),
                InvoiceLeyendaDireccionEntregaEnrichRule::fill(self),
                InvoiceLeyendaPercepcionEnrichRule::fill(self),
                InvoiceTipoComprobanteEnrichRule::fill(self),
                InvoiceTipoOperacionEnrichRule::fill(self),
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
                NoteTipoComprobanteAfectadoEnrichRule::fill(self),
                CreditNoteTipoEnrichRule::fill(self),
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
                NoteTipoComprobanteAfectadoEnrichRule::fill(self),
                DebitNoteTipoEnrichRule::fill(self),
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

// Summary implementations

impl SummaryTrait for Invoice {
    fn summary(&mut self) {
        self.summary_common();
    }
}

impl SummaryTrait for CreditNote {
    fn summary(&mut self) {
        // self.summary_common();
    }
}

impl SummaryTrait for DebitNote {
    fn summary(&mut self) {
        // self.summary_common();
    }
}

impl<T> SummaryCommonTrait for T
where
    T: InvoiceTotalImpuestosSummaryRule
        + InvoiceTotalImporteSummaryRule
        + LeyendaIVAPSummaryRule
        + PercepcionSummaryRule,
{
    fn summary_common(&mut self) {
        let mut changed = true;

        while changed {
            let results = vec![
                InvoiceTotalImpuestosSummaryRule::summary(self),
                InvoiceTotalImporteSummaryRule::summary(self),
                LeyendaIVAPSummaryRule::summary(self),
                PercepcionSummaryRule::summary(self),
            ];

            changed = results.contains(&true);
        }
    }
}
