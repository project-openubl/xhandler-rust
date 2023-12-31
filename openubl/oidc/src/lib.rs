use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
}

impl UserClaims {
    pub fn user_id(&self) -> String {
        self.sub.clone()
    }
}
