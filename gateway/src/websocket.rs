use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    error::Error,
    packet::{Packet, TaggedPacket},
};

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

    /// Guaranteed to return a packet each call while connection is running,
    /// Will return error on invalid packet data
    pub async fn next(&mut self) -> Result<Packet, Error> {
        //TODO remove panic
        let next = self.stream.next().await.expect("Stream ended somehow")?;

        let content = match next {
            Message::Text(s) => s,
            _ => return Err(Error::InvalidPacketFormat),
        };

        let packet = Packet::from_str(&content)?;

        Ok(packet)
    }
}

pub struct GatewaySink {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl GatewaySink {
    pub fn new(sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Self {
        Self { sink }
    }

    pub async fn send(&mut self, item: impl Into<TaggedPacket>) -> Result<(), Error> {
        let packet: TaggedPacket = item.into();

        self.sink
            .send(Message::Text(packet.to_json()?))
            .await
            .map_err(|_| Error::FailedToSend)
    }
}
