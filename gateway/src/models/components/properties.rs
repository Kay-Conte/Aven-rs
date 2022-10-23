use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]

pub struct Properties {
    pub os: String,
    pub browser: String,
    pub device: String,
}
