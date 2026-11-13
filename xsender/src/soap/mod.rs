use crate::models::Status;

pub mod cdr;
pub mod envelope;
pub mod send_file_response;
pub mod verify_ticket_response;

pub const SOAP_ENV_NS: &str = "http://schemas.xmlsoap.org/soap/envelope/";

pub struct SoapFault {
    pub code: String,
    pub message: String,
}

/// Extracts the numeric SUNAT code from a SOAP fault.
///
/// SUNAT puts it in `faultcode` (e.g. `soap-env:Client.0111` or `env:Server.1033`)
/// or, for some services, as the whole `faultstring` (e.g. `0306`).
pub fn sunat_fault_code(fault_code: &str, fault_message: &str) -> Option<u32> {
    let local_code = fault_code
        .rsplit_once(':')
        .map_or(fault_code, |(_, local)| local)
        .trim();
    let numeric_part = local_code
        .strip_prefix("Client.")
        .or_else(|| local_code.strip_prefix("Server."))
        .unwrap_or(local_code);

    numeric_part
        .parse::<u32>()
        .ok()
        .or_else(|| fault_message.trim().parse::<u32>().ok())
}

pub fn sunat_fault_status(fault_code: &str, fault_message: &str) -> Status {
    sunat_fault_code(fault_code, fault_message)
        .map(Status::from_code)
        .unwrap_or(Status::Unknown)
}

#[cfg(test)]
mod tests {
    use super::{sunat_fault_code, sunat_fault_status};
    use crate::models::Status;

    #[test]
    fn fault_code_extraction() {
        assert_eq!(sunat_fault_code("soap-env:Client.0111", "msg"), Some(111));
        assert_eq!(sunat_fault_code("env:Server.1033", "msg"), Some(1033));
        assert_eq!(sunat_fault_code("S:Client.2335", ""), Some(2335));
        assert_eq!(sunat_fault_code("0151", "msg"), Some(151));
        assert_eq!(sunat_fault_code("soap-env:Client", "0306"), Some(306));
        assert_eq!(sunat_fault_code("soap-env:Client", "Bad request"), None);

        assert_eq!(sunat_fault_status("env:Client.2335", ""), Status::Rechazado);
        assert_eq!(sunat_fault_status("env:Client.0111", ""), Status::Excepcion);
        assert_eq!(sunat_fault_status("env:Client", "oops"), Status::Unknown);
    }
}
