use std::path::Path;

use xsender::prelude::*;

const BASE: &str = "tests/resources/e2e";

lazy_static::lazy_static! {
    pub static ref CLIENT: FileSender = FileSender {
        urls: Urls {
            invoice: "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string(),
            perception_retention:"https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService".to_string(),
            despatch: "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem".to_string(),
        },
        credentials: Credentials {
            username: "12345678959MODDATOS".to_string(),
            password: "MODDATOS".to_string(),
        },
    };
}

#[tokio::test]
async fn send_invoice() {
    let file_path = format!("{BASE}/12345678912-01-F001-1.xml");
    let xml_file = UblFile::from_path(Path::new(&file_path)).expect("File not found");

    let response = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    let result = match response.data {
        SendFileResponse::Ok(_, cdr) => {
            assert_eq!("0", cdr.response_code);
            assert_eq!(
                "La Factura numero F001-1, ha sido aceptada",
                cdr.description
            );
            assert_eq!(Vec::<String>::new(), cdr.notes);
            true
        }
        SendFileResponse::Ticket(_) => false,
        SendFileResponse::Error(_, _) => false,
    };

    assert!(result);
}

#[tokio::test]
async fn send_voided_documents() {
    let file_path = format!("{BASE}/12345678912-RA-20200328-1.xml");
    let xml_file = UblFile::from_path(Path::new(&file_path)).expect("File not found");

    let send_file_response = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    // Send file
    let verify_ticket_target = send_file_response
        .verify_ticket_target
        .expect("Could not determine the verify_ticket target");
    let ticket = match send_file_response.data {
        SendFileResponse::Ok(_, _) => "".to_string(),
        SendFileResponse::Ticket(ticket) => ticket,
        SendFileResponse::Error(_, _) => "".to_string(),
    };
    assert!(ticket.len() > 0);

    // Verify ticket
    let verify_ticket_response = CLIENT
        .verify_ticket(&verify_ticket_target, &ticket)
        .await
        .expect("Could not verify ticket");
    let result = match verify_ticket_response {
        VerifyTicketResponse::Ok(_, cdr, status_code) => {
            assert_eq!("0", cdr.response_code);
            assert_eq!(
                "La Comunicacion de baja RA-20200328-1, ha sido aceptada",
                cdr.description
            );
            assert_eq!(Vec::<String>::new(), cdr.notes);

            assert_eq!("0", status_code);
            true
        }
        VerifyTicketResponse::Error(_, _) => false,
    };
    assert!(result);
}
