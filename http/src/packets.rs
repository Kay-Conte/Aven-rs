use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GatewayInit {
    url: String,
    shards: u8,
    // Session start limit omitted
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum OpPacket {
    #[serde(skip)]
    Invalid {},
}
