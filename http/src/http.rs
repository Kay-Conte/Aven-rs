use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

use crate::packet::Packet;

const url: &str = "https://discord.com/api/v10";

#[derive(Clone)]
pub struct Http {
    client: Client,
}

impl Http {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bot {}", token).as_str())
                .expect("Failed to construct default header"),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to construct reqwest::Client");

        Http { client }
    }

    fn get_gateway_bot(&self) -> Packet {
        Packet::Invalid {}
    }
}
