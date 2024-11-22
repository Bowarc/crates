pub enum FutureUpdate<T> {
    Started,
    Done(T),
    Panicked,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FutureState {
    Flying,  // Not yet picked up by a worker
    Started, // Currently being executed by a worker
    Done,
    Panicked,
}

pub struct Future<T> {
    // data: std::sync::Mutex<Option<T>>,
    // state: std::sync::Mutex<FutureState>,

    // flag: std::sync::atomic::AtomicBool,
    // condvar: std::sync::Condvar,
    output: Option<T>,
    _sender: crossbeam::channel::Sender<FutureUpdate<T>>,
    receiver: crossbeam::channel::Receiver<FutureUpdate<T>>,
    state: FutureState,
}

// #[derive(Debug, Clone, Copy)]
// pub enum FutureError {
//     NoData,
//     NotDone,
//     LockError,
// }

impl<T> Future<T> {
    pub fn new(
        sender: crossbeam::channel::Sender<FutureUpdate<T>>,
        receiver: crossbeam::channel::Receiver<FutureUpdate<T>>,
    ) -> Self {
        Self {
            output: None::<T>,

            // I HATE this,
            // I need to store the sender here too else the comunication will close when the worker is done
            // and the Future won't be able to read the stream :c
            _sender: sender,
            receiver,
            state: FutureState::Flying,
        }
    }

    fn recv_one(&self, blocking: bool) -> Option<FutureUpdate<T>> {
        use crossbeam::channel::{RecvError, TryRecvError};
        if blocking {
            match self.receiver.recv() {
                Ok(msg) => Some(msg),
                Err(RecvError) => None,
            }
        } else {
            match self.receiver.try_recv() {
                Ok(msg) => Some(msg),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => panic!("Could not read the receiver"),
            }
        }
    }

    fn update(&mut self, blocking: bool) {
        // loop {
        let Some(msg) = self.recv_one(blocking) else {
            return;
        };

        match msg {
            FutureUpdate::Started => {
                println!("Received: Stated");
                self.state = FutureState::Started
            }
            FutureUpdate::Done(output) => {
                println!("Received: Done");
                self.output = Some(output);
                self.state = FutureState::Done;
            }
            FutureUpdate::Panicked => {
                println!("Received: Panicked");
                self.state = FutureState::Panicked
            }
        }
        // }
    }

    pub fn state(&mut self) -> FutureState {
        self.update(false);
        self.state
    }

    pub fn is_done(&mut self) -> bool {
        self.update(false);
        matches!(self.state, FutureState::Done) || matches!(self.state, FutureState::Panicked)
    }

    pub fn wait(&mut self) {
        while self.state != FutureState::Done && self.state != FutureState::Panicked {
            self.update(true);
            println!("Update done");
        }
    }

    pub fn output(self) -> Option<T> {
        self.output
    }
}
