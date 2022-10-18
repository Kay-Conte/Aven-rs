use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::op_codes;

#[derive(Debug, Serialize, Deserialize)]

pub struct TaggedPacket {
    op: u64,
    d: Packet,
}

impl TaggedPacket {
    pub fn new(op: u64, d: Packet) -> Self {
        Self { op: op.into(), d }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    pub fn identify(token: String, properties: String, shard: [u8; 2]) -> Self {
        TaggedPacket::from(Packet::Identify(Identify {
            token,
            properties,
            shard,
        }))
    }

    pub fn heartbeat(sequence: Option<u64>) -> Self {
        TaggedPacket::from(Packet::Heartbeat(Heartbeat(sequence)))
    }
}

impl From<Packet> for TaggedPacket {
    fn from(other: Packet) -> Self {
        TaggedPacket::new(other.op_code(), other)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Representation of the data field of a discord Op packet
///
/// Each variant contains a tuple with the inner value
/// being a struct representing the specific op code's data

pub enum Packet {
    Dispatch(Dispatch),
    Heartbeat(Heartbeat),
    Identify(Identify),
    Hello(Hello),
    HearbeatAck(HeartbeatAck),
}

impl Packet {
    fn op_code(&self) -> u64 {
        match self {
            Packet::Dispatch(_) => op_codes::DISPATCH,
            Packet::Heartbeat(_) => op_codes::HEARTBEAT,
            Packet::Identify(_) => op_codes::IDENTIFY,
            Packet::Hello(_) => op_codes::HELLO,
            Packet::HearbeatAck(_) => op_codes::HEARTBEAT_ACK,
        }
    }

    pub fn from_str(str: &str) -> Result<Packet, Error> {
        let mut json: serde_json::Value = serde_json::from_str(str)?;

        let obj = json.as_object_mut().ok_or(Error::InvalidPacketFormat)?;

        let op = obj
            .get("op")
            .ok_or(Error::InvalidPacketFormat)?
            .as_u64()
            .ok_or(Error::InvalidPacketFormat)?;

        let d = obj
            .get("d")
            .map(|inner| inner.to_owned())
            .ok_or(Error::InvalidPacketFormat)?;

        let packet: Packet = match op {
            op_codes::DISPATCH => match serde_json::from_value::<Dispatch>(d) {
                Ok(packet) => packet,
                Err(_) => return Err(Error::InvalidPacketFormat),
            }
            .into(),
            op_codes::HEARTBEAT => match serde_json::from_value::<Heartbeat>(d) {
                Ok(packet) => packet,
                Err(_) => return Err(Error::InvalidPacketFormat),
            }
            .into(),
            op_codes::IDENTIFY => match serde_json::from_value::<Identify>(d) {
                Ok(packet) => packet,
                Err(_) => return Err(Error::InvalidPacketFormat),
            }
            .into(),

            op_codes::HELLO => match serde_json::from_value::<Hello>(d) {
                Ok(packet) => packet,
                Err(_) => return Err(Error::InvalidPacketFormat),
            }
            .into(),

            _ => return Err(Error::InvalidOpCode),
        };

        Ok(packet)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dispatch {}

impl From<Dispatch> for Packet {
    fn from(other: Dispatch) -> Self {
        Self::Dispatch(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat(Option<u64>);

impl From<Heartbeat> for Packet {
    fn from(other: Heartbeat) -> Self {
        Self::Heartbeat(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identify {
    pub token: String,
    pub properties: String,
    pub shard: [u8; 2],
}

impl From<Identify> for Packet {
    fn from(other: Identify) -> Self {
        Self::Identify(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u32,
}

impl From<Hello> for Packet {
    fn from(other: Hello) -> Self {
        Self::Hello(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatAck {}

impl From<HeartbeatAck> for Packet {
    fn from(other: HeartbeatAck) -> Self {
        Self::HearbeatAck(other)
    }
}
