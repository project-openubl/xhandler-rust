/// Set of URL where the company should send his documents
#[derive(Clone)]
pub struct Urls {
    pub invoice: String,
    pub perception_retention: String,
    pub despatch: String,
}

/// Credentials of the company for sending documents
#[derive(Clone, Default)]
pub struct Credentials {
    /// SOL user, e.g. "20123456789MODDATOS"
    pub username: String,
    pub password: String,
    /// API client of the REST services (guias de remision), from SUNAT's SOL menu
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

/// SUNAT service to look up documents already sent (getStatus, getStatusCdr)
pub const BILL_CONSULT_SERVICE_URL: &str =
    "https://e-factura.sunat.gob.pe/ol-it-wsconscpegem/billConsultService";
/// SUNAT service to validate documents (validaCDPcriterios, verificaCPEarchivo)
pub const BILL_VALID_SERVICE_URL: &str =
    "https://e-factura.sunat.gob.pe/ol-it-wsconsvalidcpe/billValidService";

/// A document already sent to SUNAT, to look up in the consult services
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConsultDocument {
    pub ruc: String,
    /// Catalog1, e.g. "01"
    pub tipo_comprobante: String,
    pub serie: String,
    pub numero: u32,
}

/// Criteria to validate a document with `validaCDPcriterios`
#[derive(Clone, Debug, serde::Serialize)]
pub struct ValidateCdp {
    #[serde(flatten)]
    pub document: ConsultDocument,
    /// Catalog6
    pub tipo_documento_identidad_receptor: String,
    pub numero_documento_identidad_receptor: String,
    /// "dd/MM/yyyy"
    pub fecha_emision: String,
    pub importe_total: String,
    pub numero_autorizacion: Option<String>,
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
    pub const DESPATCH_ADVICE: &'static str = "DespatchAdvice";
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

/// Outcome of a document according to the SUNAT response code
/// (CDR `ResponseCode`, SOAP fault code or ticket `statusCode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// 0, or 4000+ (accepted with observations)
    Accepted,
    /// 2000-3999
    Rejected,
    /// 99 and 100-1999: the document was not processed and can be sent again
    Exception,
    /// 98: ticket still being processed
    InProgress,
    /// Any other code
    Unknown,
}

impl Status {
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::Accepted,
            98 => Self::InProgress,
            99..=1999 => Self::Exception,
            2000..=3999 => Self::Rejected,
            4000.. => Self::Accepted,
            _ => Self::Unknown,
        }
    }

    /// Parses a numeric code, ignoring surrounding whitespace and leading zeros.
    pub fn from_code_str(code: &str) -> Self {
        code.trim()
            .parse::<u32>()
            .map(Self::from_code)
            .unwrap_or(Self::Unknown)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Accepted => "ACEPTADO",
            Self::Rejected => "RECHAZADO",
            Self::Exception => "EXCEPCION",
            Self::InProgress => "EN_PROCESO",
            Self::Unknown => "UNKNOWN",
        };
        write!(f, "{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::Status;

    #[test]
    fn status_from_code() {
        assert_eq!(Status::from_code(0), Status::Accepted);
        assert_eq!(Status::from_code(1), Status::Unknown);
        assert_eq!(Status::from_code(97), Status::Unknown);
        assert_eq!(Status::from_code(98), Status::InProgress);
        assert_eq!(Status::from_code(99), Status::Exception);
        assert_eq!(Status::from_code(100), Status::Exception);
        assert_eq!(Status::from_code(1999), Status::Exception);
        assert_eq!(Status::from_code(2000), Status::Rejected);
        assert_eq!(Status::from_code(2335), Status::Rejected);
        assert_eq!(Status::from_code(3999), Status::Rejected);
        assert_eq!(Status::from_code(4000), Status::Accepted);
        assert_eq!(Status::from_code(4252), Status::Accepted);
        assert_eq!(Status::from_code_str(" 0111 "), Status::Exception);
        assert_eq!(Status::from_code_str("abc"), Status::Unknown);
    }
}
