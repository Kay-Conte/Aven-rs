# Aven-rs

![](https://github.com/Kay-Conte/Aven-rs/banner.png)

This project is intended to be a user friendly interface for the discord bot api. The project is in it's early stages and the api will be **VERY** unstable for the time being as I am deciding on a structure for the project.

## Roadmap

- [x] Http and Gateway connections - Finishing packet modeling and deserialization

- [ ] Heartbeat over gateway - Currently working on

- [ ] Sharding

- [ ] Data Models

- [ ] Event Models

- [ ] Event Handling

- [ ] Message Components

- [ ] Interaction support

## Currently suported

There are currently no supported features, this framework does not function in it's current state.

This project is actively being maintained. I primarily program using an offline repo so do not worry if you don't see a push for a while during the initial stages.

## Getting Started

```rust
use aven::prelude::*;

#[derive(Default)]
pub struct Cache {}

#[derive(Default)]
pub struct Bot;

impl Bot {
    fn new() -> Self {
        Self {}
    }
}

impl Application for Bot {
    type Cache = Cache;

    fn token(&self) -> String {
        std::env::var("DISCORD_TOKEN").expect("Failed to get discord token")
    }

    async fn message(&self, ctx: Context<Self::Cache>, msg: Message) {
        println!("Handling message");
    }
}

fn main() {
    let bot = Bot::new();

    let application = bot.run();

    if application.is_err() {
        println!("Application failed to run");
    }
}



```
