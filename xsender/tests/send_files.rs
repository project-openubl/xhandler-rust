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

#[serial_test::serial]
#[tokio::test]
async fn send_invoice() {
    let file_path = format!("{BASE}/12345678912-01-F001-1.xml");
    let xml_file = UblFile::from_path(Path::new(&file_path)).expect("File not found");

    let result = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    let result = match result.response {
        SendFileAggregatedResponse::Cdr(_, cdr_metadata) => {
            assert_eq!("0", cdr_metadata.response_code);
            assert_eq!(
                "La Factura numero F001-1, ha sido aceptada",
                cdr_metadata.description
            );
            assert_eq!(Vec::<String>::new(), cdr_metadata.notes);
            true
        }
        SendFileAggregatedResponse::Ticket(_) => false,
        SendFileAggregatedResponse::Error(_) => false,
    };

    assert!(result);
}

#[serial_test::serial]
#[tokio::test]
async fn send_voided_documents() {
    let file_path = format!("{BASE}/12345678912-RA-20200328-1.xml");
    let xml_file = UblFile::from_path(Path::new(&file_path)).expect("File not found");

    let file_result = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    // Send file
    let _verify_ticket_target = file_result
        .verify_ticket_target
        .expect("Could not determine the verify_ticket target");
    let ticket = match file_result.response {
        SendFileAggregatedResponse::Cdr(_, _) => "".to_string(),
        SendFileAggregatedResponse::Ticket(ticket) => ticket,
        SendFileAggregatedResponse::Error(_) => "".to_string(),
    };
    assert!(!ticket.is_empty());

    // TODO uncomment this when the beta server works again
    // Verify ticket
    // let ticket_result = CLIENT
    //     .verify_ticket(&verify_ticket_target, &ticket)
    //     .await
    //     .expect("Could not verify ticket");
    // let result = match ticket_result.response {
    //     VerifyTicketAggregatedResponse::Cdr(status, cdr_metadata) => {
    //         assert_eq!("0", status.status_code);

    //         assert_eq!("0", cdr_metadata.response_code);
    //         assert_eq!(
    //             "La Comunicacion de baja RA-20200328-1, ha sido aceptada",
    //             cdr_metadata.description
    //         );
    //         assert_eq!(Vec::<String>::new(), cdr_metadata.notes);

    //         true
    //     }
    //     VerifyTicketAggregatedResponse::Error(_) => false,
    // };
    // assert!(result);
}
