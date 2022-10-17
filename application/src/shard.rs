use tokio::task::JoinHandle;

pub struct Shard {
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
}
