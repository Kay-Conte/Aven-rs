mod events;
mod packet;
mod shard;

use shard::Shard;
use tokio_tungstenite;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

const GATEWAY_URL: &str = "wss://gateway.discord.gg/";

pub struct Gateway {
    token: String,
    shard: Shard,
}

impl Gateway {
    pub fn new(token: String) -> Self {
        Self {
            token,
            shard: Shard::new(),
        }
    }

    pub async fn connect() {}

    pub async fn send() {}
}

#[cfg(test)]
mod test {

    use futures_util::{SinkExt, StreamExt};
    use tokio::{runtime, task};
    use tokio_tungstenite::connect_async;

    use super::*;
    #[test]
    fn gateway_connection() {
        let rt = runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .expect("Failed to construct runtime");

        rt.block_on(async move {
            let (ws_stream, _) = connect_async(GATEWAY_URL)
                .await
                .expect("Failed to connect to Discord gateway");

            let (mut write, mut read) = ws_stream.split();

            let handle = task::spawn(async move {
                while let m = read.next().await {
                    
                }
                println!("Read loop closing");
            });

            let _ = write
                .send(tokio_tungstenite::tungstenite::Message::Text(
                    "Test".to_string(),
                ))
                .await;

            let _ = handle.await;
        });
    }
}
