use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]

pub struct Token {
    token: String,
}

impl From<String> for Token {
    fn from(other: String) -> Self {
        Token { token: other }
    }
}
