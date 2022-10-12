const API_BASE: &str = "https://discord.com/api/v10/";

#[derive(Clone)]
pub struct Connection {
    token: String,
}

impl Connection {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
