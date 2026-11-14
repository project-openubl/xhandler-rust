use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{builder, sender, signer};

#[derive(Deserialize)]
struct Request {
    op: String,
    #[serde(default)]
    input: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Response {
    Ok(Value),
    Error(Error),
}

/// An operation failure, reported to the host as `{"kind", "message"}`.
#[derive(Debug, Serialize)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// The request is not `{"op", "input"}` or the op does not exist.
    InvalidRequest,
    /// The input does not match the op (missing fields, wrong types...).
    InvalidInput,
    /// The document has invalid values, e.g. an unknown catalog code.
    Validation,
    /// The XML could not be rendered.
    Render,
    /// The private key or certificate can't be read (e.g. a wrong keystore
    /// password), or the certificate is not the one of the private key.
    InvalidKey,
    /// The answer of SUNAT can't be read, e.g. an HTML error page.
    Response,
    /// SUNAT's front proxy refused the request (rate limiting): retry it
    /// after a pause.
    RateLimited,
}

impl Error {
    pub fn new(kind: ErrorKind, error: impl Into<anyhow::Error>) -> Self {
        // `{:#}` keeps the causes, e.g. the template filter that failed.
        let message = format!("{:#}", error.into());
        Self { kind, message }
    }
}

pub type OpResult = Result<Value, Error>;

/// Parses input JSON into the type an op expects.
pub fn parse_input<T: serde::de::DeserializeOwned>(input: Value) -> Result<T, Error> {
    serde_json::from_value(input).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

/// Runs a JSON request and returns the JSON response. Never fails: errors are
/// part of the response.
pub fn invoke(request: &[u8]) -> Vec<u8> {
    let result = serde_json::from_slice::<Request>(request)
        .map_err(|e| Error::new(ErrorKind::InvalidRequest, e))
        .and_then(|request| dispatch(&request.op, request.input));

    let response = match result {
        Ok(value) => Response::Ok(value),
        Err(error) => Response::Error(error),
    };
    serde_json::to_vec(&response).expect("a response is always serializable")
}

fn dispatch(op: &str, input: Value) -> OpResult {
    match op {
        "version" => Ok(env!("CARGO_PKG_VERSION").into()),
        "build" => builder::build(input),
        "enrich" => builder::enrich(input),
        "sign" => signer::sign(input),
        "prepare_send_file" => sender::prepare_send_file(input),
        "prepare_send_pack" => sender::prepare_send_pack(input),
        "verify_ticket_target" => sender::verify_ticket_target(input),
        "request" => sender::request(input),
        "parse" => sender::parse(input),
        _ => Err(Error::new(
            ErrorKind::InvalidRequest,
            anyhow::anyhow!("unknown op: {op}"),
        )),
    }
}
