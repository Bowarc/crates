/*
    The idea is to boot up a threadpool with how many threads you want,
    You get clone-able handle to it.
    You can use that handle to send one or multiple closure(s) to be exectuted in another thread.
    Doing that will return you a Future (one per closure submited) that is used to get the return data of that closure.
    You can .wait future to block the current thread until that closure has returned


    Not done yet:
        What if a thread panics ?



    Threads lifetime:
        Threads are started by the ThreadPool::new method
        and are closed by dropping the ThreadPool.sender
        which will make their receiver fail on `.recv()`

*/

mod command;
mod future;
mod task;
mod worker;

use command::Command;
use future::{Future, FutureState};
use task::Task;
use worker::Worker;

#[derive(Clone)]
pub struct ThreadPool {
    // workers: std::sync::Arc<Vec<Worker>>,
    sender: std::sync::Arc<crossbeam::channel::Sender<Command>>,

    flying_tasks_count: std::sync::Arc<std::sync::atomic::AtomicU16>,
}

impl ThreadPool {
    pub fn new(thread_count: u16) -> Self {
        let (s, r) = crossbeam::channel::unbounded();

        let mut index: u16 = 0;
        // let workers =
        (0..thread_count).for_each(|_| {
            index += 1;
            Worker::new(index - 1, r.clone());
        });
        // .collect::<Vec<_>>()
        // .into();

        Self {
            // workers,
            sender: std::sync::Arc::new(s),
            flying_tasks_count: std::sync::Arc::new(std::sync::atomic::AtomicU16::new(0)),
        }
    }

    pub fn run<O: Clone + Send + 'static, F: FnOnce() -> O + Send + 'static>(
        &self,
        task: F,
    ) -> std::sync::Arc<Future<O>> {
        self.flying_tasks_count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        let future = std::sync::Arc::new(Future::<O>::default());

        let f = future.clone();
        let ftc = self.flying_tasks_count.clone();

        self.sender
            .send(Command::Job(Box::new(move || {
                f.set_started();
                let output = task();
                f.set_done(output);
                ftc.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            })))
            .unwrap();

        future
    }
    pub fn flying_tasks_count(&self) -> u16 {
        self.flying_tasks_count
            .load(std::sync::atomic::Ordering::Acquire)
    }
}
