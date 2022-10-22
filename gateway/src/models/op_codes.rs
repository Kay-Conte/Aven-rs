//! Used to construct packets with OpCode in the Deserializer<'_> implementation

//? If custom discriminants on tuple or struct enums are ever added, this can be replaced with serde_repr.

pub const DISPATCH: u64 = 0;
pub const HEARTBEAT: u64 = 1;
pub const IDENTIFY: u64 = 2;
pub const PRESENCE_UPDATE: u64 = 3;
pub const VOICE_STATE_UPDATE: u64 = 4;
pub const RESUME: u64 = 6;
pub const RECONNECT: u64 = 7;
pub const REQUEST_GUILD_MEMBERS: u64 = 8;
pub const INVALID_SESSION: u64 = 9;
pub const HELLO: u64 = 10;
pub const HEARTBEAT_ACK: u64 = 11;
