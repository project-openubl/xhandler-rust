use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub name: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
}
