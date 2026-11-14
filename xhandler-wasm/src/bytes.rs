use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Bytes that cross the JSON boundary as standard base64.
pub struct Base64(pub Vec<u8>);

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        STANDARD
            .decode(text)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl From<Base64> for Value {
    fn from(bytes: Base64) -> Self {
        STANDARD.encode(bytes.0).into()
    }
}
