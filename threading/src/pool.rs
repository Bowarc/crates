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

    // Used to send Tasks to the ThreadPool
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
            worker::Worker::new(index - 1, r.clone());
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

        self.sender
            .send(Command::Job(Box::new(move || {
                f.set_state(FutureState::Started);
                let output = task();
                f.set_output(output);
                f.set_state(FutureState::Done);
            })))
            .unwrap();

        future
    }
}
