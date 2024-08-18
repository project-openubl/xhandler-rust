/// Set of URL where the company should send his documents
#[derive(Clone)]
pub struct Urls {
    pub invoice: String,
    pub perception_retention: String,
    pub despatch: String,
}

/// Credentials of the company for sending documents
#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Defines where the document should be sent
#[derive(Debug, PartialEq)]
pub enum SendFileTarget {
    /// URL and Method
    Soap(String, SoapFileTargetAction),

    /// URL and Method
    Rest(String, RestFileTargetAction),
}

/// Defines where the document should be sent
#[derive(Debug, PartialEq)]
pub enum VerifyTicketTarget {
    /// URL and Method
    Soap(String),

    /// URL and Method
    Rest(String),
}

#[derive(Debug, PartialEq)]
pub enum SoapFileTargetAction {
    /// Sends a single file
    Bill,
    /// Send summary-documents or voided-documents
    Summary,
    /// Sends a set of files all together
    Pack,
}

#[derive(Debug, PartialEq)]
pub enum RestFileTargetAction {
    SendDocument,
}

pub struct DocumentType {}

impl DocumentType {
    pub const INVOICE: &'static str = "Invoice";
    pub const CREDIT_NOTE: &'static str = "CreditNote";
    pub const DEBIT_NOTE: &'static str = "DebitNote";
    pub const VOIDED_DOCUMENTS: &'static str = "VoidedDocuments";
    pub const SUMMARY_DOCUMENTS: &'static str = "SummaryDocuments";
    pub const DESPATCH_ADVICE: &'static str = "DespatchAdvise";
    pub const PERCEPTION: &'static str = "Perception";
    pub const RETENTION: &'static str = "Retention";
}

pub(crate) struct Catalog1 {}

#[allow(dead_code)]
impl Catalog1 {
    pub const FACTURA: &'static str = "01";
    pub const BOLETA: &'static str = "03";
    pub const NOTA_CREDITO: &'static str = "07";
    pub const NOTA_DEBITO: &'static str = "08";
    pub const GUIA_REMISION_REMITENTE: &'static str = "09";
    pub const RETENCION: &'static str = "20";
    pub const GUIA_REMISION_TRANSPORTISTA: &'static str = "31";
    pub const PERCEPCION: &'static str = "40";
}
