use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

use super::{
    components::{Properties, Token},
    op_codes, Dispatch, Hello, Identify,
};

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Packet {
    pub op: u64,
    #[serde(flatten)]
    pub d: Data,
}

impl Packet {
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| e.into())
    }

    pub fn from_str(_str: &str) -> Result<Self, Error> {
        todo!()
    }
}

impl From<Data> for Packet {
    fn from(other: Data) -> Self {
        let op = match other {
            Data::Dispatch(_) => op_codes::DISPATCH,
            Data::Hello(_) => op_codes::HELLO,
            Data::Identify(_) => op_codes::IDENTIFY,
        };

        Packet { op, d: other }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Data {
    #[serde(rename = "d")]
    Dispatch(Dispatch),

    #[serde(rename = "d")]
    Hello(Hello),

    #[serde(rename = "d")]
    Identify(Identify),
}

impl Data {
    /// Returns op_code of Data variant
    pub fn op_code(&self) -> u64 {
        todo!()
    }

    pub fn identify(token: impl Into<Token>, properties: Properties, shard: [u64; 2]) -> Self {
        Self::Identify(Identify::new(token, properties, shard))
    }
}

impl<'de> Deserialize<'de> for Packet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PacketVisitor::new())
    }
}

pub struct PacketVisitor {
    op: Option<u64>,
    d: Option<Value>,
    s: Option<Value>,
    t: Option<Value>,
}

impl PacketVisitor {
    fn new() -> Self {
        Self {
            op: None,
            d: None,
            s: None,
            t: None,
        }
    }
}

impl<'de> Visitor<'de> for PacketVisitor {
    type Value = Packet;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize Packet")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            match key.as_str() {
                "op" => {
                    self.op = Some(
                        value
                            .as_u64()
                            .ok_or(serde::de::Error::custom("Invalid value at op"))?,
                    )
                }
                "d" => self.d = Some(value),
                "s" => self.s = Some(value),
                "t" => self.t = Some(value),
                _ => {}
            }
        }

        let op = self.op.ok_or(serde::de::Error::custom(
            "Invalid packet format, no op code",
        ))?;

        let value = self.d.ok_or(serde::de::Error::custom(
            "Invalid packet format, Missing d field",
        ))?;

        match op {
            op_codes::HELLO => {
                let d: Hello = serde_json::from_value(value).map_err(|_| {
                    serde::de::Error::custom("Invalid packet format, Failed to parse d field")
                })?;

                let packet: Packet = Packet {
                    op,
                    d: Data::Hello(d),
                };

                Ok(packet)
            }
            _ => Err(serde::de::Error::custom("Invalid op code")),
        }
    }
}
