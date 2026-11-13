use anyhow::anyhow;
use std::str::FromStr;

use crate::soap::{SoapFault, SOAP_ENV_NS};
use crate::xml_utils::walk_xml;

/// Response from SUNAT while getting a ticket status through SOAP
pub enum VerifyTicketXmlResponse {
    Status(VerifyTicketXmlStatus),
    /// Error
    Fault(SoapFault),
}

pub struct VerifyTicketXmlStatus {
    /// CDR zip in base64. SUNAT omits it while the ticket is still being
    /// processed (status code 98).
    pub cdr_base64: Option<String>,
    pub status_code: String,
}

impl FromStr for VerifyTicketXmlResponse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cdr_base64: Option<String> = None;
        let mut status_code: Option<String> = None;
        let mut fault_code: Option<String> = None;
        let mut fault_message: Option<String> = None;

        walk_xml(s, |path, text| {
            let text = text.trim().to_string();
            let in_status_response = path.iter().any(|e| e.is_local("getStatusResponse"));

            match path {
                [.., parent, element]
                    if parent.is(SOAP_ENV_NS, "Fault") && element.is_local("faultcode") =>
                {
                    fault_code.get_or_insert(text);
                }
                [.., parent, element]
                    if parent.is(SOAP_ENV_NS, "Fault") && element.is_local("faultstring") =>
                {
                    fault_message.get_or_insert(text);
                }
                [.., element] if in_status_response && element.is_local("content") => {
                    if !text.is_empty() {
                        cdr_base64.get_or_insert(text);
                    }
                }
                [.., element] if in_status_response && element.is_local("statusCode") => {
                    status_code.get_or_insert(text);
                }
                _ => {}
            }
        })?;

        match (status_code, fault_code) {
            (Some(status_code), _) => Ok(Self::Status(VerifyTicketXmlStatus {
                cdr_base64,
                status_code,
            })),
            (_, Some(code)) => Ok(Self::Fault(SoapFault {
                code,
                message: fault_message.unwrap_or_default(),
            })),
            _ => Err(anyhow!(
                "Could not extract the expected data from ticket response"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;

    use crate::soap::verify_ticket_response::VerifyTicketXmlResponse;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn read_ok_response() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/get_status_response_ok.xml"))
            .expect("Could not read file");
        let response =
            VerifyTicketXmlResponse::from_str(&file_content).expect("Could not read response");

        let result = match response {
            VerifyTicketXmlResponse::Status(data) => {
                (data.cdr_base64.unwrap_or_default(), data.status_code)
            }
            VerifyTicketXmlResponse::Fault(_) => ("".to_string(), "".to_string()),
        };

        let expected_cdr = "UEsDBBQAAgAIAK07lVcAAAAAAgAAAAAAAAAGAAAAZHVtbXkvAwBQSwMEFAACAAgArTuVV2PumuIyBAAACA0AAB8AAABSLTEyMzQ1Njc4OTEyLVJBLTIwMjAwMzI4LTEueG1stVdRT+M4EH7fXxGVh5X2LjhJoYUoZFUo3FULPba0gPbNJEMbLrFzttMWfv2OkyZNS9C2K53gwZn55puZz2MbvK/LJDbmIGTE2VnLPrRaBrCAhxGbnrUm4yvzpPXV/+RR4fbSNI4CqhA4AplyJsHAYCbPWplgLqcyki6jCUhXphBEzyuwmz3FrgxmkFB3KUN3wOY8CsB0WkW4S8WeDA2VrNlgqfaku+BJwtnlUgHTKuAnUgJTck0aPAW/RXqO8KCRkP4eYW86FTClCppIQ9yKmVKpS8hisThctA+5mBLHsixinRLEhDKaHpRoyWla4YtE8hBd2p4H6gUBNoeYp0CqJJi8CoOljFUO1mZpUhaaKsJeqiRlnzJjVH3YZwoiqzd7p9FNvdol8fKjXm3yeHN9l1OVWGSBZdpQNDqymAoTvQKk3nzZ8j2cIHdyfl0NhCzHvMFXWGqzw3ClfO8ummIHmaiOyA77gsdMh0E4YM/c/2QY3gVlnKFOcfSWa3UDasZDoxdPuYjULPlQAtvStNhXYAb2ETt4QLQeIK1hi+TcVYU7k1pHZa1mwgUcCElNOaPHtrOiHMEzCLw9wJiMBlouNKJ5LCiTz1wksjDUTb9MuyFROYyhKcvqi9R7ku4iEBKS7cq9fjQFqfZUDBU5qOtU8dzTOAP/hS7D/iWx5peB9eMvMumyf9KnvmN/z0bf528L/kYerkZXr7Ph+ViSm5voPusOX5aTq6Pn3n8PCfs7kT+GycPL203YW9BR6jyO/7gNF2dnHqln0ftDqg3CUSObs1afiCLiy62I5nj6jH/h1fh8Dore4lHF6wyE+mwwrows/VLQ1KK8b/Cac3qPx9ZpnyparHRUceaReYjXQGgEa9OKv0iIDDX+7eCcbSBlBuIORETjukUT709fi825Ct5hljyB2J9tI7qeoCyXrJUhlVprHXHdfKeQ95fPO5P0PXyrtOm+eNMHfd85tDzyzprjLjKpeLK6XdBol9BtR47WgK7Vtjt2t9M5alt2ga3cukvsC0jte4yvgW9Zbv67wlf2HFY+4TrSdyynbdqO6ay4N5wb8IK46zqnrtPZBK+4aeDWRF1Vqi13k2FvXKu9AnLxekuFei1s+XIQovbVY1XROJbdxh/n9Ph4TUQ+jiodxZDpgHxVq6TwkC0k+ag4PNuRonHVYE8pGsySfFC0X0+EYDReH/liMEYD/2BLA20rEjUEkV8lI9s6609gIYj/R0rSmGAEAUTznXPaTvvouNM9ObWdnXM2pOjzINMqlINX1lJ95UO50hJTjHomDrhltZ0TsxrwtXtjvi94iPO9Odi5LUf1QQYiSvPyrqmBT1em/1AI8NsIwXiiL9TYyPanMaOGjEJu0ABSRUNaUNeJyj7rzaxb3Jij5mYqFZuiCgmjNEL7jtvUMffZnQ1q0rw/pPkfGf8nUEsBAgAAFAACAAgArTuVVwAAAAACAAAAAAAAAAYAAAAAAAAAAAAAAAAAAAAAAGR1bW15L1BLAQIAABQAAgAIAK07lVdj7priMgQAAAgNAAAfAAAAAAAAAAEAAAAAACYAAABSLTEyMzQ1Njc4OTEyLVJBLTIwMjAwMzI4LTEueG1sUEsFBgAAAAACAAIAgQAAAJUEAAAAAA==";
        assert_eq!(expected_cdr, result.0);
        assert_eq!("0", result.1);
    }

    #[test]
    fn read_in_progress_response_without_cdr() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/get_status_response_in_progress.xml"))
                .expect("Could not read file");
        let response =
            VerifyTicketXmlResponse::from_str(&file_content).expect("Could not read response");
        match response {
            VerifyTicketXmlResponse::Status(data) => {
                assert_eq!(data.cdr_base64, None);
                assert_eq!(data.status_code, "98");
            }
            VerifyTicketXmlResponse::Fault(_) => panic!("Expected a status"),
        }
    }

    #[test]
    fn read_fault_with_any_prefix() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/get_status_response_fault.xml"))
            .expect("Could not read file");
        let response =
            VerifyTicketXmlResponse::from_str(&file_content).expect("Could not read response");
        match response {
            VerifyTicketXmlResponse::Fault(fault) => {
                assert_eq!(fault.code, "S:Client.0127");
                assert_eq!(fault.message, "El ticket no existe");
            }
            VerifyTicketXmlResponse::Status(_) => panic!("Expected a fault"),
        }
    }
}
