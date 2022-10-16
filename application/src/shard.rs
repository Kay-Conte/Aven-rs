use std::collections::HashMap;

use tokio::sync::mpsc;

struct Shard {
    mpsc: (mpsc::Sender<String>, mpsc::Receiver<String>),
}

impl Shard {
    pub async fn init(url: String) -> Self {
        
        
        let (sender, reciever) = mpsc::channel(1024);


        
        Shard {
            mpsc: (sender, reciever),
        }
    }

    fn run() {}
}

pub struct ShardManager {
    shards: HashMap<u32, Shard>,
}

impl ShardManager {}
