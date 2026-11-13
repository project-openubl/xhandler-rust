use anyhow::anyhow;
use std::str::FromStr;

use crate::fault::SunatFault;
use crate::soap::SOAP_ENV_NS;
use crate::xml_walker::walk_xml;

/// `<status>` answer shared by getStatus (tickets and consult),
/// validaCDPcriterios and verificaCPEarchivo; getStatusCdr calls it `<statusCdr>`
pub enum StatusXmlResponse {
    Status(StatusXml),
    Fault(SunatFault),
}

pub struct StatusXml {
    pub status_code: String,
    pub status_message: Option<String>,
    /// CDR zip in base64, when the answer carries one
    pub cdr_base64: Option<String>,
}

impl FromStr for StatusXmlResponse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut status_code: Option<String> = None;
        let mut status_message: Option<String> = None;
        let mut cdr_base64: Option<String> = None;
        let mut fault_code: Option<String> = None;
        let mut fault_message: Option<String> = None;

        walk_xml(s, |path, text| {
            let [.., parent, element] = path else {
                return;
            };
            let text = text.trim().to_string();

            if parent.is(SOAP_ENV_NS, "Fault") {
                if element.is_local("faultcode") {
                    fault_code.get_or_insert(text);
                } else if element.is_local("faultstring") {
                    fault_message.get_or_insert(text);
                }
            } else if (parent.is_local("status") || parent.is_local("statusCdr"))
                && !text.is_empty()
            {
                let slot = match element.local_name.as_str() {
                    "statusCode" => &mut status_code,
                    "statusMessage" => &mut status_message,
                    "content" => &mut cdr_base64,
                    _ => return,
                };
                slot.get_or_insert(text);
            }
        })?;

        match (status_code, fault_code) {
            (Some(status_code), _) => Ok(Self::Status(StatusXml {
                status_code,
                status_message,
                cdr_base64,
            })),
            (_, Some(code)) => Ok(Self::Fault(SunatFault {
                code,
                message: fault_message.unwrap_or_default(),
            })),
            _ => Err(anyhow!("Could not extract the status from the response")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;

    use crate::soap::status_response::StatusXmlResponse;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn read_ok_response() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/get_status_response_ok.xml"))
            .expect("Could not read file");
        let response = StatusXmlResponse::from_str(&file_content).expect("Could not read response");

        let result = match response {
            StatusXmlResponse::Status(data) => {
                (data.cdr_base64.unwrap_or_default(), data.status_code)
            }
            StatusXmlResponse::Fault(_) => ("".to_string(), "".to_string()),
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
        let response = StatusXmlResponse::from_str(&file_content).expect("Could not read response");
        match response {
            StatusXmlResponse::Status(data) => {
                assert_eq!(data.cdr_base64, None);
                assert_eq!(data.status_code, "98");
            }
            StatusXmlResponse::Fault(_) => panic!("Expected a status"),
        }
    }

    #[test]
    fn read_fault_with_any_prefix() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/get_status_response_fault.xml"))
            .expect("Could not read file");
        let response = StatusXmlResponse::from_str(&file_content).expect("Could not read response");
        match response {
            StatusXmlResponse::Fault(fault) => {
                assert_eq!(fault.code, "S:Client.0127");
                assert_eq!(fault.message, "El ticket no existe");
            }
            StatusXmlResponse::Status(_) => panic!("Expected a fault"),
        }
    }

    #[test]
    fn read_validation_response() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/valida_cdp_response.xml"))
            .expect("Could not read file");
        let StatusXmlResponse::Status(status) = StatusXmlResponse::from_str(&file_content).unwrap()
        else {
            panic!("Expected a status");
        };
        assert_eq!(status.status_code, "0");
        assert_eq!(
            status.status_message.as_deref(),
            Some("El comprobante existe y está aceptado.")
        );
        assert_eq!(status.cdr_base64, None);
    }

    #[test]
    fn read_status_cdr_response() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/get_status_cdr_response.xml"))
            .expect("Could not read file");
        let StatusXmlResponse::Status(status) = StatusXmlResponse::from_str(&file_content).unwrap()
        else {
            panic!("Expected a status");
        };
        assert_eq!(status.status_code, "0004");
        assert_eq!(status.cdr_base64.as_deref(), Some("UEsDBA=="));
    }
}
