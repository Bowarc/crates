#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FutureState {
    Flying,
    Started,
    Done,
}

pub struct Future<T: Clone> {
    data: std::sync::Mutex<Option<T>>,
    state: std::sync::Mutex<FutureState>,

    flag: std::sync::atomic::AtomicBool,
    condvar: std::sync::Condvar,
}

#[derive(Debug, Clone, Copy)]
pub enum FutureError {
    NoData,
    NotDone,
    LockError,
}

impl<T: Clone> Future<T> {
    pub fn changed(&self) -> bool {
        use std::sync::atomic::Ordering;
        self.flag.load(Ordering::Acquire)
    }

    pub fn wait(&self) -> Result<(), FutureError> {
        use std::sync::atomic::Ordering;

        let guard = self
            .condvar
            .wait_while(self.state.lock().unwrap(), |state| {
                *state != FutureState::Done
            })
            .unwrap();

        // let mut guard = self.state.lock().unwrap();
        // while !self.changed() {
        //     guard = self.condvar.wait(guard).unwrap();
        // }

        if *guard != FutureState::Done {
            drop(guard); // Deadlocks :c
            self.flag.store(false, Ordering::Relaxed);
            self.wait().unwrap();
        }

        Ok(())
    }

    // It's actually required to not get &mut self, as we store it in an Arc and we only do Atomics operations
    pub fn set_state(&self, new_state: FutureState) {
        use std::sync::atomic::Ordering;
        trace!("Future set to: {new_state:?}");
        {
            let mut guard = self.state.lock().unwrap();
            *guard = new_state;

            self.flag.store(true, Ordering::Release)
        }

        self.condvar.notify_one()
    }

    pub fn set_output(&self, output: T) {
        *self.data.lock().unwrap() = Some(output);
    }

    pub fn output(&self) -> Result<T, FutureError> {
        if *self.state.lock().unwrap() != FutureState::Done {
            // Return an error or something
            return Err(FutureError::NotDone);
        }

        let Ok(mut lock) = self.data.lock() else {
            return Err(FutureError::LockError);
        };

        let Some(data) = lock.take() else {
            return Err(FutureError::NoData);
        };

        Ok(data)
    }
}

impl<T: Clone> Default for Future<T> {
    fn default() -> Self {
        use std::sync::{atomic::AtomicBool, Condvar, Mutex};
        Self {
            data: Mutex::new(None::<T>),
            state: Mutex::new(FutureState::Flying),

            flag: AtomicBool::new(false),
            condvar: Condvar::new(),
        }
    }
}
