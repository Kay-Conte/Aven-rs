use serde::{Deserialize, Serialize};

use super::{
    components::{Properties, Token},
    packet::Data,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Identify {
    token: Token,
    properties: Properties,
    shard: [u64; 2],
}

impl From<Identify> for Data {
    fn from(other: Identify) -> Self {
        Data::Identify(other)
    }
}
