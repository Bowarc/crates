#[macro_use]
extern crate log;

mod channel;
pub mod pool;
pub use pool::ThreadPool;

pub type ArcThreadPool = std::sync::Arc<ThreadPool>;

pub use channel::Channel;
