use anyhow::anyhow;
use std::str::FromStr;

use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::soap::SoapFault;

/// Response from SUNAT while getting a ticket status through SOAP
pub enum VerifyTicketXmlResponse {
    /// (CDR in base64, status_code)
    Status(VerifyTicketXmlStatus),

    /// Error
    Fault(SoapFault),
}

pub struct VerifyTicketXmlStatus {
    pub cdr_base64: String,
    pub status_code: String,
}

impl FromStr for VerifyTicketXmlResponse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event_reader = EventReader::from_str(s);

        enum Wrapper {
            StatusResponse,
            Fault,
        }

        let mut current_wrapper: Option<Wrapper> = None;
        let mut current_text: String = String::from("");

        // Send bill data
        let mut cdr_base64: Option<String> = None;
        let mut status_code: Option<String> = None;

        // Fault data
        let mut fault_code: Option<String> = None;
        let mut fault_message: Option<String> = None;

        fn set_wrapper(
            is_start: bool,
            name: &OwnedName,
            mut current_wrapper: Option<Wrapper>,
        ) -> Option<Wrapper> {
            let prefix = name.prefix.as_deref();
            let local_name = name.local_name.as_str();

            match (prefix, local_name) {
                (Some("ns2"), "getStatusResponse") => {
                    current_wrapper = if is_start {
                        Some(Wrapper::StatusResponse)
                    } else {
                        None
                    }
                }
                (Some("soap-env"), "Fault") => {
                    current_wrapper = if is_start { Some(Wrapper::Fault) } else { None }
                }
                _ => {}
            };

            current_wrapper
        }

        for event in event_reader {
            match event {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_wrapper = set_wrapper(true, &name, current_wrapper);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    let local_name = name.local_name.as_str();

                    match (&current_wrapper, local_name) {
                        (Some(Wrapper::StatusResponse), "content") => {
                            if cdr_base64.is_none() {
                                cdr_base64 = Some(current_text.clone());
                            }
                        }
                        (Some(Wrapper::StatusResponse), "statusCode") => {
                            if status_code.is_none() {
                                status_code = Some(current_text.clone());
                            }
                        }
                        (Some(Wrapper::Fault), "faultcode") => {
                            if fault_code.is_none() {
                                fault_code = Some(current_text.clone());
                            }
                        }
                        (Some(Wrapper::Fault), "faultstring") => {
                            if fault_message.is_none() {
                                fault_message = Some(current_text.clone());
                            }
                        }
                        _ => {}
                    };

                    current_wrapper = set_wrapper(false, &name, current_wrapper);
                }
                Ok(XmlEvent::Characters(characters)) => {
                    current_text = characters;
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
        }

        match (cdr_base64, status_code, fault_code, fault_message) {
            (Some(cdr_base64), Some(status_code), _, _) => {
                Ok(Self::Status(VerifyTicketXmlStatus {
                    cdr_base64,
                    status_code,
                }))
            }
            (_, _, Some(code), Some(message)) => Ok(Self::Fault(SoapFault { code, message })),
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
            VerifyTicketXmlResponse::Status(data) => (data.cdr_base64, data.status_code),
            VerifyTicketXmlResponse::Fault(_) => ("".to_string(), "".to_string()),
        };

        let expected_cdr = "UEsDBBQAAgAIAK07lVcAAAAAAgAAAAAAAAAGAAAAZHVtbXkvAwBQSwMEFAACAAgArTuVV2PumuIyBAAACA0AAB8AAABSLTEyMzQ1Njc4OTEyLVJBLTIwMjAwMzI4LTEueG1stVdRT+M4EH7fXxGVh5X2LjhJoYUoZFUo3FULPba0gPbNJEMbLrFzttMWfv2OkyZNS9C2K53gwZn55puZz2MbvK/LJDbmIGTE2VnLPrRaBrCAhxGbnrUm4yvzpPXV/+RR4fbSNI4CqhA4AplyJsHAYCbPWplgLqcyki6jCUhXphBEzyuwmz3FrgxmkFB3KUN3wOY8CsB0WkW4S8WeDA2VrNlgqfaku+BJwtnlUgHTKuAnUgJTck0aPAW/RXqO8KCRkP4eYW86FTClCppIQ9yKmVKpS8hisThctA+5mBLHsixinRLEhDKaHpRoyWla4YtE8hBd2p4H6gUBNoeYp0CqJJi8CoOljFUO1mZpUhaaKsJeqiRlnzJjVH3YZwoiqzd7p9FNvdol8fKjXm3yeHN9l1OVWGSBZdpQNDqymAoTvQKk3nzZ8j2cIHdyfl0NhCzHvMFXWGqzw3ClfO8ummIHmaiOyA77gsdMh0E4YM/c/2QY3gVlnKFOcfSWa3UDasZDoxdPuYjULPlQAtvStNhXYAb2ETt4QLQeIK1hi+TcVYU7k1pHZa1mwgUcCElNOaPHtrOiHMEzCLw9wJiMBlouNKJ5LCiTz1wksjDUTb9MuyFROYyhKcvqi9R7ku4iEBKS7cq9fjQFqfZUDBU5qOtU8dzTOAP/hS7D/iWx5peB9eMvMumyf9KnvmN/z0bf528L/kYerkZXr7Ph+ViSm5voPusOX5aTq6Pn3n8PCfs7kT+GycPL203YW9BR6jyO/7gNF2dnHqln0ftDqg3CUSObs1afiCLiy62I5nj6jH/h1fh8Dore4lHF6wyE+mwwrows/VLQ1KK8b/Cac3qPx9ZpnyparHRUceaReYjXQGgEa9OKv0iIDDX+7eCcbSBlBuIORETjukUT709fi825Ct5hljyB2J9tI7qeoCyXrJUhlVprHXHdfKeQ95fPO5P0PXyrtOm+eNMHfd85tDzyzprjLjKpeLK6XdBol9BtR47WgK7Vtjt2t9M5alt2ga3cukvsC0jte4yvgW9Zbv67wlf2HFY+4TrSdyynbdqO6ay4N5wb8IK46zqnrtPZBK+4aeDWRF1Vqi13k2FvXKu9AnLxekuFei1s+XIQovbVY1XROJbdxh/n9Ph4TUQ+jiodxZDpgHxVq6TwkC0k+ag4PNuRonHVYE8pGsySfFC0X0+EYDReH/liMEYD/2BLA20rEjUEkV8lI9s6609gIYj/R0rSmGAEAUTznXPaTvvouNM9ObWdnXM2pOjzINMqlINX1lJ95UO50hJTjHomDrhltZ0TsxrwtXtjvi94iPO9Odi5LUf1QQYiSvPyrqmBT1em/1AI8NsIwXiiL9TYyPanMaOGjEJu0ABSRUNaUNeJyj7rzaxb3Jij5mYqFZuiCgmjNEL7jtvUMffZnQ1q0rw/pPkfGf8nUEsBAgAAFAACAAgArTuVVwAAAAACAAAAAAAAAAYAAAAAAAAAAAAAAAAAAAAAAGR1bW15L1BLAQIAABQAAgAIAK07lVdj7priMgQAAAgNAAAfAAAAAAAAAAEAAAAAACYAAABSLTEyMzQ1Njc4OTEyLVJBLTIwMjAwMzI4LTEueG1sUEsFBgAAAAACAAIAgQAAAJUEAAAAAA==";
        assert_eq!(expected_cdr, result.0);
        assert_eq!("0", result.1);
    }
}
