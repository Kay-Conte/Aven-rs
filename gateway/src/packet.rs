use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Packet {
    #[serde(rename = "0")]
    Dispatch {},

    #[serde(rename = "1")]
    Heartbeat {},

    #[serde(rename = "10")]
    Hello {},

    #[serde(rename = "11")]
    HearbeatAck {},
}
