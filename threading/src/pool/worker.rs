pub struct Worker {
    id: u16,
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: u16, receiver: crossbeam::channel::Receiver<super::Command>) -> Self {
        let thread = std::thread::Builder::new()
            .name(format!("Worker {id}"))
            .spawn(move || {
                loop {
                    let Ok(message) = receiver.recv() else {
                        trace!("[{id}] Could not receive any command, Exiting");
                        break;
                    };

                    match message {
                        super::command::Command::Job(f) => {
                            trace!("[{id}] got a job; executing.");
                            f.call_box();
                            trace!("[{id}] Job's done.");
                        }
                        super::command::Command::Exit => break,
                    }
                }

                println!("[{id}] Exited");
            })
            .unwrap();

        Self { id, thread }
    }
}
