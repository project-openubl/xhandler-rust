use crate::enricher::Defaults;
use crate::enricher::rules::phase1fill::detalle::detalles::DetallesEnrichRule;
use crate::enricher::rules::phase1fill::fecha_emision::FechaEmisionEnrichRule;
use crate::enricher::rules::phase1fill::firmante::FirmanteEnrichRule;
use crate::enricher::rules::phase1fill::icb_tasa::ICBTasaEnrichRule;
use crate::enricher::rules::phase1fill::igv_tasa::IGVTasaEnrichRule;
use crate::enricher::rules::phase1fill::invoice::anticipos::InvoiceAnticiposEnrichRule;
use crate::enricher::rules::phase1fill::invoice::descuentos::InvoiceDescuentosEnrichRule;
use crate::enricher::rules::phase1fill::invoice::forma_de_pago::{
    InvoiceFormaDePagoEnrichRule, InvoiceFormaDePagoTipoRule, InvoiceFormaDePagoTotalRule,
};
use crate::enricher::rules::phase1fill::invoice::leyenda::{
    InvoiceLeyendaDetraccionEnrichRule, InvoiceLeyendaDireccionEntregaEnrichRule,
    InvoiceLeyendaPercepcionEnrichRule,
};
use crate::enricher::rules::phase1fill::invoice::tipo_comprobante::InvoiceTipoComprobanteEnrichRule;
use crate::enricher::rules::phase1fill::invoice::tipo_operacion::InvoiceTipoOperacionEnrichRule;
use crate::enricher::rules::phase1fill::ivap_tasa::IVAPTasaEnrichRule;
use crate::enricher::rules::phase1fill::moneda::MonedaEnrichRule;
use crate::enricher::rules::phase1fill::note::creditnote::tipo_nota::CreditNoteTipoEnrichRule;
use crate::enricher::rules::phase1fill::note::debitnote::tipo_nota::DebitNoteTipoEnrichRule;
use crate::enricher::rules::phase1fill::note::tipo_comprobante_afectado::NoteTipoComprobanteAfectadoEnrichRule;
use crate::enricher::rules::phase1fill::proveedor::ProveedorEnrichRule;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait FillTrait {
    fn fill(&mut self, defaults: &Defaults);
}

trait FillCommonTrait {
    fn fill_common(&mut self, defaults: &Defaults);
}

trait FillInvoiceTrait {
    fn fill_invoice(&mut self, defaults: &Defaults);
}

trait FillCreditNoteTrait {
    fn enrich_credit_note(&mut self, defaults: &Defaults);
}

trait FillDebitNoteTrait {
    fn enrich_debit_note(&mut self, defaults: &Defaults);
}

impl FillTrait for Invoice {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
        self.fill_invoice(defaults);
    }
}

impl FillTrait for CreditNote {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
    }
}

impl FillTrait for DebitNote {
    fn fill(&mut self, defaults: &Defaults) {
        self.fill_common(defaults);
    }
}

impl<T> FillCommonTrait for T
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
    fn fill_common(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
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

impl<T> FillInvoiceTrait for T
where
    T: InvoiceAnticiposEnrichRule
        + InvoiceDescuentosEnrichRule
        + InvoiceFormaDePagoEnrichRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceFormaDePagoTipoRule
        + InvoiceLeyendaDetraccionEnrichRule
        + InvoiceLeyendaDireccionEntregaEnrichRule
        + InvoiceLeyendaPercepcionEnrichRule
        + InvoiceTipoComprobanteEnrichRule
        + InvoiceTipoOperacionEnrichRule,
{
    fn fill_invoice(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                InvoiceAnticiposEnrichRule::fill(self),
                InvoiceDescuentosEnrichRule::fill(self),
                InvoiceFormaDePagoEnrichRule::fill(self),
                InvoiceFormaDePagoTotalRule::fill(self),
                InvoiceFormaDePagoTipoRule::fill(self),
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

impl<T> FillCreditNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoEnrichRule + CreditNoteTipoEnrichRule,
{
    fn enrich_credit_note(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                NoteTipoComprobanteAfectadoEnrichRule::fill(self),
                CreditNoteTipoEnrichRule::fill(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> FillDebitNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoEnrichRule + DebitNoteTipoEnrichRule,
{
    fn enrich_debit_note(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = [
                NoteTipoComprobanteAfectadoEnrichRule::fill(self),
                DebitNoteTipoEnrichRule::fill(self),
            ];

            changed = results.contains(&true);
        }
    }
}
