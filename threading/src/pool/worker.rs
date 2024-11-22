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
    id: u16,
    receiver: crossbeam::channel::Receiver<super::Task>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name(format!("Worker {id}"))
        .spawn(move || {
            loop {
                let Ok(task) = receiver.recv() else {
                    trace!("[{id}] Could not receive any command, Exiting");
                    break;
                };

                trace!("[{id}] got a job; executing.");
                task.call_box(id);
                trace!("[{id}] Job's done.");
            }

            trace!("Worker {id} exited");
        })
        .unwrap()
}
