use serde::{Deserialize, Serialize};

use super::packet::Data;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hello {
    heartbeat_interval: u64,
}

impl From<Hello> for Data {
    fn from(other: Hello) -> Self {
        Data::Hello(other)
    }
}
