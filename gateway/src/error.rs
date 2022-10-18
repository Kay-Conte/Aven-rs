pub enum Error {
    InvalidPacketFormat,
    PacketParseError,
    FailedToConnect,
    FailedToSend,
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::PacketParseError
    }
}
