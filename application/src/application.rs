use std::sync::Arc;

use crate::{
    connection::Connection,
    error::Error,
    shard::{Shard, ShardManager},
};
use async_trait::async_trait;
use aven_executor::DiscordRuntime;
use aven_gateway::{
    init_split_gateway,
    models::{components::Properties, packet::Data, Packet},
};
use aven_http::Http;
use aven_models::Message;
use tokio::{
    sync::RwLock,
    task::{self, JoinHandle},
    time,
};

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
///
/// Data *may* be stored in Self though note this data will be consumed and wrapped in an Arc after calling run
///
/// Data typically should be stored in your application::AppCache
/// which will be wrapped in an Arc<RwLock<>> for interior mutability
#[async_trait]
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

    /// This method is called once and is expected to return a valid discord token
    ///
    /// The token returned will be the token used for all further requests
    fn token(&self) -> String;

    /// This method expects system properties
    ///
    /// this method can be assumed to be called once
    ///
    /// Used to identify system with discord api
    fn properties(&self) -> Properties {
        Properties {
            browser: "Aven".to_string(),
            os: "Aven_os".to_string(),
            device: "Aven_sys".to_string(),
        }
    }

    /// This method is called when a shard recieves a message.
    ///
    /// This method can be omitted
    ///
    /// This method will likely become async using async-trait to
    /// allow sending messages and calling other asynchronous tasks from this call.
    async fn message(&self, ctx: Context<Self::AppCache>, msg: Message);

    /// Call this method to run your application, Constructs an executor, connects with the discord api,
    /// and handles all incoming events based on trait methods
    ///
    /// Sharding is handled transparently through this method
    ///
    /// This method is not intended to be overwritten
    ///
    /// Note: this function consumes ownership of Self
    fn run(self) -> Result<(), Error> {
        let application = Arc::new(self);

        let token = application.token();
        let properties = application.properties();

        let http = Http::new(&token);

        let cache = Arc::new(RwLock::new(Self::AppCache::default()));

        let rt = DiscordRuntime::new()?;

        rt.block_on(async move {
            let context = Context::new(application.token(), Self::AppCache::default());

            let mut shard_manager = ShardManager::new();

            for _ in [0..1] {
                // TODO replace clone calls if possible
                let application = application.clone();
                let token = token.clone();

                // Todo Find alternative to clone
                let properties = properties.clone();

                let http = http.clone();
                let context = context.clone();

                let task: JoinHandle<()> = task::spawn(async move {
                    let gateway_init = match http.get_gateway().await {
                        Ok(init) => init,
                        Err(_) => return,
                    };

                    let (mut sink, mut stream) = match init_split_gateway(gateway_init.url).await {
                        Ok(gateway) => gateway,
                        Err(_) => return,
                    };

                    let event_loop = task::spawn(async move {
                        let context = context;
                        while let Ok(packet) = stream.next().await {
                            match packet {
                                // Packet::Hello(hello) => {
                                //     task::spawn(async move {
                                //         let duration = time::Duration::from_millis(
                                //             hello.heartbeat_interval.into(),
                                //         );

                                //         loop {
                                //             tokio::time::sleep(duration).await;
                                //         }
                                //     });
                                // }
                                _ => {}
                            }

                            task::spawn(async move {});
                            // Event loop
                            // handle events
                        }
                    });

                    let _ = sink
                        .send(Data::identify(token, application.properties(), [0, 1]))
                        .await;

                    let _ = event_loop.await;
                });

                let shard = Shard::new(task);

                shard_manager.push(shard);
            }

            shard_manager.block().await;
        });

        Ok(())
    }
}
