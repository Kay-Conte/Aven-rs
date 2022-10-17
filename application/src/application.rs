use std::sync::Arc;

use crate::{
    connection::Connection,
    shard::{Shard, ShardManager},
};
use aven_executor::DiscordRuntime;
use aven_http::Http;
use aven_models::Message;
use tokio::{sync::RwLock, task};

/// This struct is the global application context that is sent to
pub struct Context<C> {
    conn: Connection,
    cache: Arc<RwLock<C>>,
}

impl<C> Clone for Context<C> {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl<C> Context<C> {
    pub fn new(token: String, cache: C) -> Self {
        Self {
            conn: Connection::new(token),
            cache: Arc::new(RwLock::new(cache)),
        }
    }
}

/// Application trait is the main trait used to build a discord application
pub trait Application
where
    Self: Sized + Send + Sync + 'static,
{
    /// Usually a custom struct with any data you will need to access accross calls.
    /// This type is constructed only once and passed to all relevant function calls.
    ///
    /// This type will be constructed from the Default trait and wrapped in an Arc<RwLock<T>>
    ///
    /// For lots of concurrent data access consider wrapping individual fields in Arc<RwLock<T>> additionally to reduce bottleneck
    type AppCache: Default + Send + Sync;

    /// This method is called once on Application::run() and should return a valid discord token.
    fn token(&self) -> String;

    /// This method is called when a shard recieves a message.
    ///
    /// This method can be omitted
    ///
    /// This method will likely become async using async-trait to
    /// allow sending messages and calling other asynchronous tasks from this call.
    fn message(&self, ctx: Context<Self::AppCache>, msg: Message);

    /// This method is not intended to be overwritten
    /// but it can be if you wish to implement or integrate with a custom executor.
    ///
    /// Note: this function consumes ownership of Self
    fn run(self: Self) -> Result<(), std::io::Error> {
        let application = Arc::new(self);

        let token = application.token();

        let http = Http::new(&token);

        let cache = Arc::new(RwLock::new(Self::AppCache::default()));

        let rt = DiscordRuntime::new()?;

        rt.block_on(async move {
            let context = Context::new(application.token(), Self::AppCache::default());

            let mut shard_manager = ShardManager::new();

            for _ in [0..4] {
                // TODO remove clone call for opt
                let http = http.clone();

                let task = task::spawn(async move {
                    let gateway_init = http.get_gateway().await;
                    
                    
                    // connect gateway bot over wss

                    // init event loop

                    // handle events
                });

                let shard = Shard::new(task);

                shard_manager.push(shard);
            }

            shard_manager
        });

        Ok(())
    }
}
