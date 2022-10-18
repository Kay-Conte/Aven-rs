use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

use super::op_codes;

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Dispatch {},

    Hello(Hello),
    Identify {},
}

impl Packet {
    pub fn to_json(&self) -> Result<String, Error> {
        todo!()
    }

    pub fn from_str(str: &str) -> Result<Self, Error> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    pub heartbeat_interval: u64,
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

        let packet = match self.op.ok_or(serde::de::Error::custom(
            "Invalid packet format, no op code",
        ))? {
            op_codes::HELLO => {
                let value = self.d.ok_or(serde::de::Error::custom(
                    "Invalid packet format, Missing d field",
                ))?;

                let packet = serde_json::from_value(value).map_err(|_| {
                    serde::de::Error::custom("Invalid packet format, Missing d field")
                })?;

                Packet::Hello(packet)
            }
            _ => return Err(serde::de::Error::custom("Invalid op code")),
        };

        Ok(packet)
    }
}
