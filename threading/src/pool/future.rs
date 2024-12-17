/// Represents the possible states of a `Future`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FutureState {
    /// The future has not yet been picked up by a worker.
    Flying,
    /// The future is currently being executed by a worker.
    Started,
    /// The future has completed execution successfully.
    Done,
    /// The future has encountered a panic during execution.
    Panicked,
}
/// A representation of a value that may not be available yet, allowing for asynchronous computation.
///
/// The `Future` struct provides a way to track the state of a computation and retrieve its result once it is available.
pub struct Future<T> {
    data: parking_lot::Mutex<Option<T>>,
    state: parking_lot::Mutex<FutureState>,

    condvar: parking_lot::Condvar,
}

impl<T> Future<T> {
    // It's actually required to not get &mut self, as we store it in an Arc and we only do Atomics operations
    #[inline]
    fn set_state(&self, new_state: FutureState) {
        *self.state.lock() = new_state;

        let _has_a_thread_been_woken_up = self.condvar.notify_one();
    }

    pub(crate) fn set_started(&self) {
        self.set_state(FutureState::Started);
    }

    pub(crate) fn set_done(&self, output: T) {
        *self.data.lock() = Some(output);
        
        self.set_state(FutureState::Done);
    }

    pub(crate) fn set_panicked(&self) {
        self.set_state(FutureState::Panicked)
    }

    /// Checks if the future has completed execution (either done or panicked).
    pub fn is_done(&self) -> bool {
        let state = self.state();
        matches!(state, FutureState::Done) || matches!(state, FutureState::Panicked)
    }

    /// Retrieves the current [FutureState].
    pub fn state(&self) -> FutureState {
        *self.state.lock()
    }

    /// Waits for the future to complete execution.
    ///
    /// This method blocks the current thread until the future is either done or panicked.
    pub fn wait(&self) {
        let mut state_lock = self.state.lock();
        if matches!(*state_lock, FutureState::Done) || matches!(*state_lock, FutureState::Panicked)
        {
            return;
        }

        self.condvar.wait_while(&mut state_lock, |state| {
            *state != FutureState::Done && *state != FutureState::Panicked
        });

        drop(state_lock);

        self.wait();
    }

    /// Retrieves the output of the future.
    ///
    /// # Panics if
    /// - The task has panicked
    /// - The task has not yet returned
    pub fn output(&self) -> T {
        assert!(
            self.state() == FutureState::Done,
            "Can't read the output of a task that has not completed successfully"
        );

        match self.data.lock().take(){
            Some(output) => output,
            None => panic!("The output of this future has already been moved out"),
        }
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
