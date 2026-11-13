use anyhow::anyhow;
use std::str::FromStr;

use crate::soap::{SoapFault, SOAP_ENV_NS};
use crate::xml_walker::walk_xml;

/// Response from the SUNAT while sending a file through SOAP
pub enum SendFileXmlResponse {
    /// Response from a successfully request
    Cdr(String),
    /// String contains the ticket code
    Ticket(String),
    /// Error
    Fault(SoapFault),
}

impl FromStr for SendFileXmlResponse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cdr_base64: Option<String> = None;
        let mut ticket: Option<String> = None;
        let mut fault_code: Option<String> = None;
        let mut fault_message: Option<String> = None;

        walk_xml(s, |path, text| {
            let [.., parent, element] = path else {
                return;
            };
            let text = text.trim().to_string();

            if parent.is_local("sendBillResponse") && element.is_local("applicationResponse") {
                cdr_base64.get_or_insert(text);
            } else if (parent.is_local("sendSummaryResponse")
                || parent.is_local("sendPackResponse"))
                && element.is_local("ticket")
            {
                ticket.get_or_insert(text);
            } else if parent.is(SOAP_ENV_NS, "Fault") && element.is_local("faultcode") {
                fault_code.get_or_insert(text);
            } else if parent.is(SOAP_ENV_NS, "Fault") && element.is_local("faultstring") {
                fault_message.get_or_insert(text);
            }
        })?;

        match (cdr_base64, ticket, fault_code) {
            (Some(cdr_base64), _, _) => Ok(Self::Cdr(cdr_base64)),
            (_, Some(ticket), _) => Ok(Self::Ticket(ticket)),
            (_, _, Some(code)) => Ok(Self::Fault(SoapFault {
                code,
                message: fault_message.unwrap_or_default(),
            })),
            _ => Err(anyhow!("Could not read the expected data")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;

    use crate::soap::send_file_response::SendFileXmlResponse;

    const RESOURCES: &str = "resources/test";

    #[test]
    fn read_ok_response() {
        let file_content = fs::read_to_string(format!("{RESOURCES}/bill_service_response_ok.xml"))
            .expect("Could not read file");
        let response =
            SendFileXmlResponse::from_str(&file_content).expect("Could not read response");

        let cdr = match response {
            SendFileXmlResponse::Cdr(cdr_base64) => cdr_base64,
            SendFileXmlResponse::Ticket(_) => "".to_string(),
            SendFileXmlResponse::Fault(_) => "".to_string(),
        };

        let expected_cdr = "UEsDBBQAAgAIAEwklVcAAAAAAgAAAAAAAAAGAAAAZHVtbXkvAwBQSwMEFAACAAgATCSVVwwGkuIxBAAAGQ0AABsAAABSLTEyMzQ1Njc4OTEyLTAxLUYwMDEtMS54bWy1V2FP2zwQ/r5fEZUPk6Y3OElbWKPQqVDGsgFitDC0byY52uhN7WA7peXXv+e4SdMStHbSq/LBuXvuubvHZ1sEXxaz1JqDkAlnJy330GlZwCIeJ2xy0robf7U/t770PwRU+IMsS5OIKgTegsw4k2BhMJMnrVwwn1OZSJ/RGUhfZhAlTyuwnz+mvoymMKP+QsZ+yOY8icD2Wibcp2JPhoZK1mywUHvSnfHZjLPzhQKmVcBPpASm5Jo0eoz+ivQU4VEjIf07wsFkImBCFTSRxrgVU6Uyn5CXl5fDl/YhFxPiOY5DnB5BTCyTyUGJlpxmFd4kkofo0vYiUC8IsDmkPANSJcHkVRgsZKoKsDZLm7LYVgn2UiUp+5Q5o+rdPjMQeb3ZkUY39eqWxIv3enXJw9XlqKAqscgCi6yhaHTkKRU2egVIvfmy1Q9wgvy708tqIGQ55g0+Y6nNDsOV6gejZIId5KI6IjvsCx4zHQZxyJ54/4NlBWeUcYY6pclrodUVqCmPrUE64SJR09m7EriOpsW+IjtyO+zgF6L1AGkNW6TgrircmdTplLXaMy7gQEhqyyntut6K8haeQODtAdbdbajlQiOax4Iy+cTFTBpD3fTHtBsSlcMY27Ks3qTek3QXgZCQbFceDJMJSLWnYqjIQV2niueepjn0H16PRPsBEvXt+ntGw3m4hMvrQXZ+OoB0zKYv98+/w2V4/hS/dm/uur+PB/C8OLvoEX41ypxzGf5KLn9cPF9E38/vczn8OVVikRz9PDkJSD2L3h9SbRCOGtmctfpEmIhPNyKZ4+mz/oWl9fEUFL3Bo4rXGQj10WJcWXn2ydDUooIfsCw4g4eu0xtSRc1KR5kzj8zXeA3EVrQ2rfhNQmSo8W8HF2yhlDmIEYiEpnWLJt6fvhZbcBne63z2CGJ/to3oeoKyXLJWhlRqrXXEdfOdQt5ePm9Msh/gW6VN9+ZND4d979AJyBtrgTvLpeKz1e2CRreEbjsKtAYcO20XJ/mo2z72DLTy6iaHeos8x+3Zrmd7nbHj+MXfClpB1hFjfC76DbDCXsDKN37F7bULbtdgN5wbcEPc8dsd3+tuglfcNPJrqq960ZbR3fVgXOuuAnKxvKFCLY2tWIYxbk71mlU0qEEbf16v210TkfejSoeZQh1QrGqVGA/ZQpL3isPDnyiaVg0OlKLRdFZMkvbrkRGMpus7wUzObdg/2NJA20yihiDyp2RkW2f9CSwG8f9ISRoT3EIEyXznnK7X7nSPjj/3XG/nnA0phjzKtQrl4JW1VF/FUK60xBRf8dWwq8le2zcG+4zHONibE13YCtQQZCSSrKjrklpfaYSyU4thHYJbJsE/1pRaMom5RSPIFI2pYavHlj3VC1+3szEzW4VXUjXBjU5JlqB9x704wvNe/fbZjY0spHk/SPN/Nv3/AFBLAQIAABQAAgAIAEwklVcAAAAAAgAAAAAAAAAGAAAAAAAAAAAAAAAAAAAAAABkdW1teS9QSwECAAAUAAIACABMJJVXDAaS4jEEAAAZDQAAGwAAAAAAAAABAAAAAAAmAAAAUi0xMjM0NTY3ODkxMi0wMS1GMDAxLTEueG1sUEsFBgAAAAACAAIAfQAAAJAEAAAAAA==";
        assert_eq!(expected_cdr, cdr);
    }

    #[test]
    fn read_fault_response() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/bill_service_response_fault.xml"))
                .expect("Could not read file");
        let response =
            SendFileXmlResponse::from_str(&file_content).expect("Could not read response");

        let fault = match response {
            SendFileXmlResponse::Cdr(_) => ("".to_string(), "".to_string()),
            SendFileXmlResponse::Ticket(_) => ("".to_string(), "".to_string()),
            SendFileXmlResponse::Fault(error) => (error.code, error.message),
        };

        assert_eq!("soap-env:Client.0111", fault.0);
        assert_eq!("No tiene el perfil para enviar comprobantes electronicos - Detalle: Rejected by policy.", fault.1);
    }

    #[test]
    fn read_ticket_response() {
        let file_content =
            fs::read_to_string(format!("{RESOURCES}/bill_service_response_ticket.xml"))
                .expect("Could not read file");
        let response =
            SendFileXmlResponse::from_str(&file_content).expect("Could not read response");

        let ticket = match response {
            SendFileXmlResponse::Cdr(_) => "".to_string(),
            SendFileXmlResponse::Ticket(ticket) => ticket,
            SendFileXmlResponse::Fault(_) => "".to_string(),
        };

        assert_eq!("1703154974517", ticket);
    }

    #[test]
    fn read_rechazo_fault_with_any_prefix() {
        let file_content = fs::read_to_string(format!(
            "{RESOURCES}/bill_service_response_fault_rechazo.xml"
        ))
        .expect("Could not read file");
        let response =
            SendFileXmlResponse::from_str(&file_content).expect("Could not read response");
        let SendFileXmlResponse::Fault(fault) = response else {
            panic!("Expected a fault");
        };
        assert_eq!(fault.code, "env:Client.2335");
        assert!(fault
            .message
            .starts_with("El documento electrónico ingresado ha sido alterado - Detalle: xxx"));
        assert_eq!(fault.status(), crate::models::Status::Rechazado);
    }
}
