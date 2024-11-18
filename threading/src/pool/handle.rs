#[derive(Clone)]
pub struct ThreadPoolHandle {
    // Used to send Tasks to the ThreadPool
    sender: std::sync::Arc<crossbeam::channel::Sender<super::Command>>,

    flying_tasks_count: std::sync::Arc<std::sync::atomic::AtomicU16>,
}

impl ThreadPoolHandle {
    pub fn new(sender: crossbeam::channel::Sender<super::Command>) -> Self {
        Self {
            sender: std::sync::Arc::new(sender),
            flying_tasks_count: std::sync::Arc::new(std::sync::atomic::AtomicU16::new(0)),
        }
    }

    pub fn run<O: Clone + Send + 'static, F: FnOnce() -> O + Send + 'static>(
        &self,
        task: F,
    ) -> std::sync::Arc<super::Future<O>> {
        self.flying_tasks_count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        let future = std::sync::Arc::new(super::Future::<O>::default());

        let f = future.clone();

        self.sender
            .send(super::Command::Job(Box::new(move || {
                f.set_state(super::FutureState::Started);
                let output = task();
                f.set_output(output);
                f.set_state(super::FutureState::Done);
            })))
            .unwrap();

        future
    }
}
