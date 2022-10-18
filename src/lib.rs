//! # Aven-rs
//!
//! This project is intended to be a user friendly interface for the discord bot api. The project is in it's early stages and the api will be **VERY** unstable for the time being as I am deciding on a structure for the project.
//!
//! ## Initial Goals
//!
//! - Open websocket connection with the discord api
//! - Support sharding
//! - Create good model representations
//! - Structure with developer friendly interface in mind
//!
//! ## Roadmap
//!
//! - ~~http and gateway connections~~
//!
//! - After initial connections with the discord api I will read over the project structure and see where to go from there. I have not worked with websockets in this manner before and am open to learning about any good practices one may recommend.
//!
//! - Side task: Considering better workspace structure as the current structure is nonsensically derived. Core is undescriptive and contains too many general modules. Executor crate may remain as a way to abstract alternative executors in the future such as a custom implementation or Smol instead of tokio.
//!
//! ## Currently suported
//!
//! There are currently no supported features, this framework does not function in it's current state.
//!
//! This project is actively being maintained. I primarily program using an offline repo so do not worry if you don't see a push for a while during the initial stages.
//!
//! ## Getting Started
//!
//! ```rust
//! use aven::prelude::*;
//!
//! #[derive(Default)]
//! pub struct Cache {}
//!
//! #[derive(Default)]
//! pub struct Bot;
//!
//! impl Bot {
//!     fn new() -> Self {
//!         Self {}
//!     }
//! }
//!
//! impl Application for Bot {
//!     type Cache = Cache;
//!
//!     fn token(&self) -> String {
//!         std::env::var("DISCORD_TOKEN").expect("Failed to get discord token")
//!     }
//!
//!     async fn message(&self, ctx: Context<Self::Cache>, msg: Message) {
//!         println!("Handling message");
//!     }
//! }
//!
//! fn main() {
//!     let bot = Bot::new();
//!
//!     let application = bot.run();
//!
//!     if application.is_err() {
//!         println!("Application failed to run");
//!     }
//! }
//!
//!
//!
//! ```

pub use aven_application;
pub use aven_models;

pub use aven_application::{
    application::{Application, Context},
    async_trait,
    error::Error,
};

/// Collection of most used items. Typical use cases should be covered by this import
pub mod prelude;
