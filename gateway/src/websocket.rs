use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{error::Error, packet::Packet};

pub async fn init_split_gateway(url: String) -> Result<(GatewaySink, GatewayStream), Error> {
    let (socket, _) = match connect_async(url).await {
        Ok(connection) => connection,
        Err(_) => return Err(Error::FailedToConnect),
    };

    let (sink, stream) = socket.split();

    Ok((GatewaySink::new(sink), GatewayStream::new(stream)))
}

pub struct GatewayStream {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl GatewayStream {
    pub fn new(stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Self {
        Self { stream }
    }

    pub async fn next(&mut self) -> Option<Packet> {
        // TODO may need to handle error, not sure of error types
        // TODO find better ergonomic representation

        if let Some(Ok(next)) = self.stream.next().await {
            let content = match next {
                Message::Text(s) => s,
                _ => return None,
            };

            if let Ok(packet) = serde_json::from_str::<Packet>(&content) {
                Some(packet)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct GatewaySink {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl GatewaySink {
    pub fn new(sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Self {
        Self { sink }
    }

    pub async fn send(&mut self, item: String) -> Result<(), Error> {
        self.sink
            .send(Message::Text(item))
            .await
            .map_err(|_| Error::FailedToSend)
    }
}
