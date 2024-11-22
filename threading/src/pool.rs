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

use future::Future;
use task::Task;
// use worker::Worker;

pub use future::FutureState;

#[derive(Clone)]
pub struct ThreadPool {
    // workers: std::sync::Arc<Vec<Worker>>,
    sender: std::sync::Arc<crossbeam::channel::Sender<Task>>,

    flying_tasks_count: std::sync::Arc<std::sync::atomic::AtomicU16>,
}

impl ThreadPool {
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

    pub fn run<
        O: Clone + Send + 'static,
        F: FnOnce() -> O + Send + std::panic::UnwindSafe + 'static,
    >(
        &self,
        task: F,
    ) -> Future<O> {
        self.flying_tasks_count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        // It will at most, be 2 message sent by the worker thread.
        // One setting the default State 'Flying' to 'Started'
        // One Setting the 'Started' state to 'Done'(with the output data) or 'Panicked' if the closure panicked
        let (data_sender, data_receiver) = crossbeam::channel::bounded(2);

        let future = Future::<O>::new(data_sender.clone(), data_receiver);

        let ftc = self.flying_tasks_count.clone();

        self.sender
            .send(Box::new(move |worker_id| {
                data_sender.send(future::FutureUpdate::Started).unwrap();
                match std::panic::catch_unwind(task) {
                    Ok(output) => {
                        data_sender
                            .send(future::FutureUpdate::Done(output))
                            .unwrap();
                    }
                    Err(payload) => {
                        data_sender.send(future::FutureUpdate::Panicked).unwrap();

                        error!("Worker {worker_id} panicked, dropping the payload");

                        /*
                            Extra-safe behavior, not sure if it's worth keeping
                            This ensure that if your payload ( panic!(payload) ) panics on drop
                            This will catch that and leak the payload to safely continue,

                            This would cause a double panic before it being dropped, which is kinda ugly,
                            ig i could try hiding it with a custom hook,
                            (is it even a good idea to silence that 2nd panic ?, silencing an error is often bad, more so if it's not your error)
                            (Can you have race conditions removing and setting panic hooks like that ?)
                            but payload that panics on drop are so rare that i don't think it's nessessary
                        */
                        {
                            // save the current hook in case it's not the default one
                            // let old_hook = std::panic::take_hook();

                            // Setting silent hook
                            // std::panic::set_hook(Box::new(|_| {}));

                            if let Err(panicking_payload) =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    drop(payload)
                                }))
                            {
                                Box::leak(panicking_payload);
                            }

                            // set the old hook back
                            // std::panic::set_hook(old_hook);
                        }
                    }
                };
                ftc.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            }))
            .unwrap();

        future
    }

    pub fn flying_tasks_count(&self) -> u16 {
        self.flying_tasks_count
            .load(std::sync::atomic::Ordering::Acquire)
    }
}
