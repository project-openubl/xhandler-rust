//! Create, sign and send the UBL XML electronic documents of SUNAT (Peru).
//!
//! - [`xbuilder`]: creates the XML from the essential data; defaults, taxes and
//!   totals are computed
//! - [`xsigner`]: signs the XML (RSA-SHA256, PEM or PKCS#12 keys)
//! - [`xsender`]: sends the files to SUNAT (SOAP services and the REST API of
//!   the guias de remision) and reads the CDR, tickets and errors
//!
//! [`prelude`] brings the three into scope.

pub mod prelude;
pub mod xbuilder;
pub mod xsender;
pub mod xsigner;
