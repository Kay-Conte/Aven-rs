use std::future::Future;

use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

pub struct DiscordRuntime {
    runtime: Runtime,
}

impl DiscordRuntime {
    pub fn new() -> Result<DiscordRuntime, std::io::Error> {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()?;

        Ok(DiscordRuntime { runtime })
    }

    pub fn block_on<F>(&self, fut: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(fut)
    }

    pub fn spawn<F>(&self, fut: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
