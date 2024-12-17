// pub struct Worker {
//     id: u16,
//     thread: std::thread::JoinHandle<()>,
// }

// impl Worker {
//     pub fn new(id: u16, receiver: crossbeam::channel::Receiver<super::Task>) -> Self {
//         let thread = start(id, receiver);

//         Self { id, thread }
//     }
// }

pub fn start(
    worker_id: u16,
    receiver: crossbeam::channel::Receiver<super::Task>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name(format!("Worker {worker_id}"))
        .spawn(move || {
            loop {
                let Ok(task) = receiver.recv() else {
                    // trace!("[{id}] Could not receive any command, Exiting");
                    break;
                };

                trace!("[{worker_id}] got a job; executing.");
                task.call(worker_id);
                trace!("[{worker_id}] Job's done.");
            }

            trace!("Worker {worker_id} exited");
        })
        .unwrap()
}

#[inline(always)]
/// This is used to move the logic outside the Threadpool::run method
pub fn task_exec_helper<F, O>(
    future: super::ArcFuture<O>,
    task: F,
    worker_id: u16,
    ftc: std::sync::Arc<std::sync::atomic::AtomicU16>,
) where
    F: FnOnce() -> O + Send + std::panic::UnwindSafe + 'static,
    O: Send + 'static,
{
    future.set_started();
    match std::panic::catch_unwind(task) {
        Ok(output) => {
            future.set_done(output);
        }
        Err(payload) => {
            future.set_panicked();

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
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(payload)))
                {
                    Box::leak(panicking_payload);
                }

                // set the old hook back
                // std::panic::set_hook(old_hook);
            }
        }
    };
    ftc.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
}
