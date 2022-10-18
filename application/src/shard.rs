use tokio::task::JoinHandle;

pub struct Shard {
    //Todo Convert to handle over Result<(), Error>
    //Todo handle Error in application run
    task: JoinHandle<()>,
}

impl Shard {
    fn run(&mut self) {}

    pub fn new(task: JoinHandle<()>) -> Self {
        Self { task }
    }
}

pub struct ShardManager {
    shards: Vec<Shard>,
}

impl ShardManager {
    pub fn new() -> Self {
        Self { shards: Vec::new() }
    }

    pub fn push(&mut self, shard: Shard) {
        self.shards.push(shard);
    }

    pub fn destruct_all(&mut self) {
        todo!()
    }

    pub async fn block(self) {
        for shard in self.shards {
            let _ = shard.task.await;
        }
    }
}
