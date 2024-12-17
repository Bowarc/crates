/*
    The idea is to boot up a threadpool with how many threads you want,
    You get clone-able handle to it.
    You can use that handle to send one or multiple closure(s) to be exectuted in another thread.
    Doing that will return you a Future (one per closure submited) that is used to get the return data of that closure.
    You can .wait the future to block the current thread until that closure has returned


    Threads lifetime:
        Threads are started by the ThreadPool::new method
        and are closed by dropping the ThreadPool.sender
        which will make their receiver fail on `.recv()`

*/

mod future;
mod task;
mod worker;

use task::Task;
// use worker::Worker;

pub use future::{Future, FutureState};

pub type ArcFuture<T> = std::sync::Arc<Future<T>>;

/// A thread pool that manages a set of worker threads to execute tasks concurrently.
///
/// The `ThreadPool` allows you to submit closures to be executed in separate threads,  
/// and provides a way to retrieve the results of those closures through [Future] objects.
#[derive(Clone)]
pub struct ThreadPool {
    // workers: std::sync::Arc<Vec<Worker>>,
    sender: std::sync::Arc<crossbeam::channel::Sender<Task>>,

    flying_tasks_count: std::sync::Arc<std::sync::atomic::AtomicU16>,
}

impl ThreadPool {
    /// Creates a new `ThreadPool` with the specified number of worker threads.
    pub fn new(thread_count: u16) -> Self {
        let (s, r) = crossbeam::channel::unbounded();

        let mut index: u16 = 0;
        // let workers =
        (0..thread_count).for_each(|_| {
            index += 1;
            worker::start(index - 1, r.clone());
        });
        // .collect::<Vec<_>>()
        // .into();

        Self {
            // workers,
            sender: std::sync::Arc::new(s),
            flying_tasks_count: std::sync::Arc::new(std::sync::atomic::AtomicU16::new(0)),
        }
    }

    /// Submits a closure to be executed by a worker thread and returns a `Future` for the result.
    ///
    /// # Parameters
    /// - `task`: The closure to be executed. It must be [`Send`], [`'static`](https://doc.rust-lang.org/stable/std/keyword.static.html), and [`panic-safe`](std::panic::UnwindSafe).
    ///
    /// # Returns
    /// An [`ArcFuture<O>`] that can be used to retrieve the result of the closure once it has completed.
    pub fn run<F, O>(&self, task: F) -> ArcFuture<O>
    where
        F: FnOnce() -> O + Send + std::panic::UnwindSafe + 'static,
        O: Send + 'static,
    {
        use std::sync::Arc;

        self.flying_tasks_count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        // It will at most, be 2 message sent by the worker thread.
        // One setting the default State 'Flying' to 'Started'
        // One Setting the 'Started' state to 'Done'(with the output data) or 'Panicked' if the closure panicked
        // let (data_sender, data_receiver) = crossbeam::channel::bounded(2);

        let future = Arc::new(Future::<O>::default());

        let cloned_future = future.clone();

        let ftc = self.flying_tasks_count.clone();

        self.sender
            .send(Box::new(move |worker_id| {
                worker::task_exec_helper(future, task, worker_id, ftc);
            }))
            .unwrap();

        cloned_future
    }

    /// Returns the number of not yet exectued tasks in the pool.
    pub fn flying_tasks_count(&self) -> u16 {
        self.flying_tasks_count
            .load(std::sync::atomic::Ordering::Acquire)
    }
}
