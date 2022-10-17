use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::error::Error;

pub struct Websocket {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    token: String,
}

impl Websocket {
    pub async fn init(url: String, token: String) -> Result<Self, Error> {
        let (socket, response) = match connect_async(url).await {
            Ok(connection) => connection,
            Err(_) => return Err(Error::FailedToConnect),
        };

        Ok(Self { socket, token })
    }

    pub async fn connect() {}

    pub async fn send() {}
}
