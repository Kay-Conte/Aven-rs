pub enum Error {
    Websocket(tokio_tungstenite::tungstenite::Error),
    InvalidPacketFormat,
    InvalidOpCode,
    Serde(serde_json::Error),
    FailedToConnect,
    FailedToSend,
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(other: tokio_tungstenite::tungstenite::Error) -> Self {
        Error::Websocket(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Self {
        Self::Serde(other)
    }
}
