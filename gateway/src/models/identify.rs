use serde::{Deserialize, Serialize};

use super::{
    components::{Properties, Token},
    packet::Data,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Identify {
    pub token: Token,
    pub properties: Properties,
    pub shard: [u64; 2],
}

impl Identify {
    pub fn new(token: impl Into<Token>, properties: Properties, shard: [u64; 2]) -> Self {
        Identify {
            token: token.into(),
            properties,
            shard,
        }
    }
}

impl From<Identify> for Data {
    fn from(other: Identify) -> Self {
        Data::Identify(other)
    }
}
