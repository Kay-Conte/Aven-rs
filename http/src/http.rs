use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

use crate::{
    error::Error,
    packets::{GatewayInit, OpPacket},
};

const URL: &str = "https://discord.com/api/v10";

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

    pub async fn get_gateway(&self) -> Result<GatewayInit, Error> {
        let res = match self.client.get(format!("{}/gateway/bot", URL)).send().await {
            Ok(s) => s,
            Err(_) => return Err(Error::NoResponse),
        };

        let text = match res.text().await {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::InvalidResponse);
            }
        };

        match serde_json::from_str::<GatewayInit>(&text) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::InvalidResponse),
        }
    }
}
