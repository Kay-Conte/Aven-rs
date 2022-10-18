///

pub enum Error {
    /// General StdIo Error, Typically this is a parse error from Serde
    StdIo,

    /// Failure to retrieve gateway init packet over http
    NoGatewayInitPacket,

    /// Failure to initialize future executor and runtime
    FailedToStartRuntime,

    /// Failure to connect over wss/websocket
    FailedToInitGateway,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::StdIo
    }
}
