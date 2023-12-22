pub mod cdr;
pub mod envelope;
pub mod send_file_response;
pub mod verify_ticket_response;

pub struct SoapFault {
    pub code: String,
    pub message: String,
}
