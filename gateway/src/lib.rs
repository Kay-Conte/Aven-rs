pub mod error;
mod events;
mod packet;
mod websocket;

pub use websocket::{init_split_gateway, GatewaySink, GatewayStream};

const GATEWAY_URL: &str = "wss://gateway.discord.gg/";
