# KitCord-rs

This project is intended to be a user friendly interface for the discord bot api. The project is in it's early stages and the api will be **VERY** unstable for the time being as I am deciding on a structure for the project.

## Initial Goals

- Open websocket connection with the discord api
- Support sharding
- Create good model representations
- Structure with developer friendly interface in mind

## Roadmap

- After initial connections with the discord api I will read over the project structure and see where to go from there. I have not worked with websockets in this manner before and am open to learning about any good practices one may recommend.

- Side task: Considering better workspace structure as the current structure is nonsensically derived. Core is undescriptive and contains too many general modules. Executor crate may remain as a way to abstract alternative executors in the future such as a custom implementation or Smol instead of tokio. 

## Currently suported

There are currently no supported features, this framework does not function in it's current state.

This project is actively being maintained. I primarily program using an offline repo so do not worry if you don't see a push for a while during the initial stages.

## Getting Started

```rust
use discord_rs::prelude::*;

#[derive(Default)]
pub struct Cache {}

pub struct Bot;

impl Application for Bot {
    type Cache = Cache;

    fn token(&self) -> String {
        std::env::var("DISCORD_TOKEN").expect("Failed to get discord token")
    }

    fn message(&self, ctx: Context<Self::Cache>, msg: Message) {}
}

fn main() {
    let application = Bot::run();
}


```
