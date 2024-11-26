#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FutureState {
    Flying,  // Not yet picked up by a worker
    Started, // Currently being executed by a worker
    Done,
    Panicked,
}

pub struct Future<T> {
    data: parking_lot::Mutex<Option<T>>,
    state: parking_lot::Mutex<FutureState>,

    condvar: parking_lot::Condvar,
}

impl<T> Future<T> {
    // It's actually required to not get &mut self, as we store it in an Arc and we only do Atomics operations
    #[inline]
    fn set_state(&self, new_state: FutureState) {
        {
            let mut guard = self.state.lock();
            *guard = new_state;
        }

        let _has_a_thread_been_woken_up = self.condvar.notify_one();
    }

    pub fn set_started(&self) {
        self.set_state(FutureState::Started);
    }

    pub fn set_done(&self, output: T) {
        let mut data_lock = self.data.lock();
        self.set_state(FutureState::Done);
        *data_lock = Some(output);
    }

    pub fn set_panicked(&self) {
        self.set_state(FutureState::Panicked)
    }

    pub fn is_done(&self) -> bool {
        let state = self.state();
        matches!(state, FutureState::Done) || matches!(state, FutureState::Panicked)
    }

    pub fn state(&self) -> FutureState {
        *self.state.lock()
    }

    pub fn wait(&self) {
        let state = self.state();
        if matches!(state, FutureState::Done) || matches!(state, FutureState::Panicked) {
            return;
        }

        self.condvar.wait_while(&mut self.state.lock(), |state| {
            *state != FutureState::Done && *state != FutureState::Panicked
        });

        self.wait();
    }

    pub fn output(&self) -> Option<T> {
        if *self.state.lock() != FutureState::Done {
            return None;
        }

        self.data.try_lock()?.take()
    }
}

impl<T> Default for Future<T> {
    fn default() -> Self {
        use parking_lot::{Condvar, Mutex};

        Self {
            data: Mutex::new(None::<T>),
            state: Mutex::new(FutureState::Flying),

            condvar: Condvar::new(),
        }
    }
}
