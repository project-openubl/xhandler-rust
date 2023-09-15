use chrono::NaiveDate;

use crate::enricher::rules::fecha_emision::FechaEmisionRule;
use crate::enricher::rules::firmante::FirmanteRule;
use crate::enricher::rules::icb::ICBRule;
use crate::enricher::rules::igv::IGVRule;
use crate::enricher::rules::invoice::formadepago::{FormaDePagoRule, FormaDePagoTotalRule};
use crate::enricher::rules::invoice::leyenda::{
    LeyendaDetraccionRule, LeyendaDireccionEntregaRule, LeyendaPercepcionRule,
};
use crate::enricher::rules::invoice::tipocomprobante::TipoComprobanteRule;
use crate::enricher::rules::invoice::tipooperacion::TipoOperacionRule;
use crate::enricher::rules::ivap::IVAPRule;
use crate::enricher::rules::moneda::MonedaRule;
use crate::enricher::rules::note::creditnote::tiponota::TipoNotaCreditoRule;
use crate::enricher::rules::note::debitnote::tiponota::TipoNotaDebitoRule;
use crate::enricher::rules::note::tipocomprobanteafectado::TipoComprobanteAfectadoRule;
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

impl<T> EnrichCommonTrait for T
where
    T: FechaEmisionRule + FirmanteRule + ICBRule + IGVRule + IVAPRule + MonedaRule + ProveedorRule,
{
    fn enrich_common(&mut self, defaults: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                self.enrich_fechaemision(defaults),
                self.enrich_firmante(),
                self.enrich_icb(defaults),
                self.enrich_igv(defaults),
                self.enrich_ivap(defaults),
                self.enrich_moneda(),
                self.enrich_proveedor(),
            ];

            changed = results.contains(&true);
        }
    }
}

// Invoice
impl<T> EnrichInvoiceTrait for T
where
    T: LeyendaDetraccionRule
        + LeyendaDireccionEntregaRule
        + LeyendaPercepcionRule
        + FormaDePagoRule
        + FormaDePagoTotalRule
        + TipoComprobanteRule
        + TipoOperacionRule,
{
    fn enrich_invoice(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                self.enrich_leyenda_detraccion(),
                self.enrich_leyenda_direccionentrega(),
                self.enrich_leyenda_percepcion(),
                self.enrich_formadepago(),
                self.enrich_formadepago_total(),
                self.enrich_tipocomprobante(),
                self.enrich_tipooperacion(),
            ];

            changed = results.contains(&true);
        }
    }
}

// Note
impl<T> EnrichCreditNoteTrait for T
where
    T: TipoComprobanteAfectadoRule + TipoNotaCreditoRule,
{
    fn enrich_creditnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                self.enrich_tipo_comprobante_afectado(),
                self.enrich_tipo_nota_credito(),
            ];

            changed = results.contains(&true);
        }
    }
}

impl<T> EnrichDebitNoteTrait for T
where
    T: TipoComprobanteAfectadoRule + TipoNotaDebitoRule,
{
    fn enrich_debitnote(&mut self, _: &Defaults) {
        let mut changed = true;

        while changed {
            let results = vec![
                self.enrich_tipo_comprobante_afectado(),
                self.enrich_tipo_nota_debito(),
            ];

            changed = results.contains(&true);
        }
    }
}
