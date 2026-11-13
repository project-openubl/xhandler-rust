use crate::models::Status;

pub mod cdr;
pub mod envelope;
pub mod send_file_response;
pub mod verify_ticket_response;

pub const SOAP_ENV_NS: &str = "http://schemas.xmlsoap.org/soap/envelope/";

/// SOAP fault returned by SUNAT
pub struct SoapFault {
    /// Raw `faultcode`, e.g. `soap-env:Client.0111`
    pub code: String,
    /// `faultstring`
    pub message: String,
}

impl SoapFault {
    /// Numeric SUNAT code. SUNAT puts it in `faultcode` (e.g. `soap-env:Client.0111`
    /// → 111) or, for some services, as the whole `faultstring` (e.g. `0306`).
    pub fn sunat_code(&self) -> Option<u32> {
        let local_code = self
            .code
            .rsplit_once(':')
            .map_or(self.code.as_str(), |(_, local)| local)
            .trim();
        let numeric_part = local_code
            .strip_prefix("Client.")
            .or_else(|| local_code.strip_prefix("Server."))
            .unwrap_or(local_code);

        numeric_part
            .parse::<u32>()
            .ok()
            .or_else(|| self.message.trim().parse::<u32>().ok())
    }

    /// RECHAZADO for 2000-3999, EXCEPCION for 100-1999...
    pub fn status(&self) -> Status {
        self.sunat_code()
            .map(Status::from_code)
            .unwrap_or(Status::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::SoapFault;
    use crate::models::Status;

    fn fault(code: &str, message: &str) -> SoapFault {
        SoapFault {
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    #[test]
    fn fault_code_extraction() {
        assert_eq!(fault("soap-env:Client.0111", "msg").sunat_code(), Some(111));
        assert_eq!(fault("env:Server.1033", "msg").sunat_code(), Some(1033));
        assert_eq!(fault("S:Client.2335", "").sunat_code(), Some(2335));
        assert_eq!(fault("0151", "msg").sunat_code(), Some(151));
        assert_eq!(fault("soap-env:Client", "0306").sunat_code(), Some(306));
        assert_eq!(fault("soap-env:Client", "Bad request").sunat_code(), None);

        assert_eq!(fault("env:Client.2335", "").status(), Status::Rechazado);
        assert_eq!(fault("env:Client.0111", "").status(), Status::Excepcion);
        assert_eq!(fault("env:Client", "oops").status(), Status::Unknown);
    }
}
