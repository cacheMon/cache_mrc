use crate::Key;

mod fifo_policy;
mod lru_policy;
pub use fifo_policy::FifoPolicy;
pub use lru_policy::LruPolicy;
pub trait EvictPolicy: Send {
    fn new(capacity: u64) -> Self;
    fn get(&mut self, key: Key) -> Option<()>;
    fn put(&mut self, key: Key, size: u64);
}
