use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Packet {
    #[serde(skip)]
    Invalid {},
}

impl From<String> for Packet {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_else(|f| Packet::Invalid {})
    }
}
