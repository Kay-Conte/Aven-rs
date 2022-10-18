pub mod error;
mod events;
pub mod packet;
mod websocket;

pub use websocket::{init_split_gateway, GatewaySink, GatewayStream};