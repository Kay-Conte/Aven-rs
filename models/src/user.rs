/// Representation of a discord User irregardless of Guild, Refer to Member for additional Guild information
pub struct User {
    id: u64,
}

impl User {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
