use crate::models::Status;

/// Error answer from SUNAT: a SOAP fault, or the equivalent JSON error of the REST API
#[derive(Clone, Debug)]
pub struct SunatFault {
    /// Raw `faultcode`, e.g. `soap-env:Client.0111`
    pub code: String,
    /// `faultstring`
    pub message: String,
}

impl SunatFault {
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

    /// Status for the SUNAT code: Rejected for 2000-3999, Exception for 99-1999, ...
    pub fn status(&self) -> Status {
        self.sunat_code()
            .map(Status::from_code)
            .unwrap_or(Status::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::SunatFault;
    use crate::models::Status;

    fn fault(code: &str, message: &str) -> SunatFault {
        SunatFault {
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

        assert_eq!(fault("env:Client.2335", "").status(), Status::Rejected);
        assert_eq!(fault("env:Client.0111", "").status(), Status::Exception);
        assert_eq!(fault("env:Client", "oops").status(), Status::Unknown);
    }
}
