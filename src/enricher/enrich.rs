use crate::enricher::rules::detalle::detalles::DetallesRule;
use chrono::NaiveDate;

use crate::enricher::rules::fecha_emision::FechaEmisionRule;
use crate::enricher::rules::firmante::FirmanteRule;
use crate::enricher::rules::icbtasa::ICBTasaRule;
use crate::enricher::rules::igvtasa::IGVTasaRule;
use crate::enricher::rules::invoice::anticipos::InvoiceAnticiposRule;
use crate::enricher::rules::invoice::descuentos::InvoiceDescuentosRule;
use crate::enricher::rules::invoice::formadepago::{
    InvoiceFormaDePagoRule, InvoiceFormaDePagoTotalRule,
};
use crate::enricher::rules::invoice::leyenda::{
    InvoiceLeyendaDetraccionRule, InvoiceLeyendaDireccionEntregaRule, InvoiceLeyendaPercepcionRule,
};
use crate::enricher::rules::invoice::tipocomprobante::InvoiceTipoComprobanteRule;
use crate::enricher::rules::invoice::tipooperacion::InvoiceTipoOperacionRule;
use crate::enricher::rules::ivaptasa::IVAPTasaRule;
use crate::enricher::rules::moneda::MonedaRule;
use crate::enricher::rules::note::creditnote::tiponota::CreditNoteTipoRule;
use crate::enricher::rules::note::debitnote::tiponota::DebitNoteTipoRule;
use crate::enricher::rules::note::tipocomprobanteafectado::NoteTipoComprobanteAfectadoRule;
use crate::enricher::rules::proveedor::ProveedorRule;
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

impl EnrichTrait for Invoice {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_invoice(defaults);
    }
}

impl EnrichTrait for CreditNote {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_creditnote(defaults);
    }
}

impl EnrichTrait for DebitNote {
    fn enrich(&mut self, defaults: &Defaults) {
        self.enrich_common(defaults);
        self.enrich_debitnote(defaults);
    }
}

// Invoice
impl<T> EnrichInvoiceTrait for T
where
    T: InvoiceAnticiposRule
        + InvoiceDescuentosRule
        + InvoiceFormaDePagoRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceLeyendaDetraccionRule
        + InvoiceLeyendaDireccionEntregaRule
        + InvoiceLeyendaPercepcionRule
        + InvoiceFormaDePagoRule
        + InvoiceFormaDePagoTotalRule
        + InvoiceTipoComprobanteRule
        + InvoiceTipoOperacionRule,
{
    fn enrich_invoice(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                InvoiceAnticiposRule::enrich(self),
                InvoiceDescuentosRule::enrich(self),
                InvoiceFormaDePagoRule::enrich(self),
                InvoiceFormaDePagoTotalRule::enrich(self),
                InvoiceLeyendaDetraccionRule::enrich(self),
                InvoiceLeyendaDireccionEntregaRule::enrich(self),
                InvoiceLeyendaPercepcionRule::enrich(self),
                InvoiceTipoComprobanteRule::enrich(self),
                InvoiceTipoOperacionRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichCommonTrait for T
where
    T: DetallesRule
        + FechaEmisionRule
        + FirmanteRule
        + ICBTasaRule
        + IGVTasaRule
        + IVAPTasaRule
        + MonedaRule
        + ProveedorRule,
{
    fn enrich_common(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                DetallesRule::enrich(self),
                FechaEmisionRule::enrich(self, defaults),
                FirmanteRule::enrich(self),
                ICBTasaRule::enrich(self, defaults),
                IGVTasaRule::enrich(self, defaults),
                IVAPTasaRule::enrich(self, defaults),
                MonedaRule::enrich(self),
                ProveedorRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

// Note
impl<T> EnrichCreditNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoRule + CreditNoteTipoRule,
{
    fn enrich_creditnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                NoteTipoComprobanteAfectadoRule::enrich(self),
                CreditNoteTipoRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichDebitNoteTrait for T
where
    T: NoteTipoComprobanteAfectadoRule + DebitNoteTipoRule,
{
    fn enrich_debitnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                NoteTipoComprobanteAfectadoRule::enrich(self),
                DebitNoteTipoRule::enrich(self),
            ];

            changed = results.contains(&true);
        }
    }
}
