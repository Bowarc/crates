pub struct Channel<R, W> {
    receiver: std::sync::mpsc::Receiver<R>,
    sender: std::sync::mpsc::Sender<W>,
}

impl<R, W> Channel<R, W> {
    // i don't rly like how Read and Write have no meaning here (as you have a Sender<R> and a Receiver<W>)
    // But having the function outside is bad too
    /// Creates a new pair of inter-connected channels
    pub fn new_pair() -> (Channel<W, R>, Channel<R, W>) {
        let (sender1, receiver1) = std::sync::mpsc::channel::<R>();
        let (sender2, receiver2) = std::sync::mpsc::channel::<W>();

        let c1 = Channel {
            sender: sender1,
            receiver: receiver2,
        };
        let c2 = Channel {
            sender: sender2,
            receiver: receiver1,
        };
        (c1, c2)
    }
}

impl<R: std::cmp::PartialEq, W: std::cmp::PartialEq> Channel<R, W> {
    /// Waits for a specific message of type R to be received.
    ///
    /// # Parameters
    /// - `waited_message`: The message to wait for.
    ///
    /// # Returns
    /// - `Ok(())` if the waited message is received.
    /// - An error of type `std::sync::mpsc::RecvError` if the receiving fails.
    pub fn wait_for(&self, waited_message: R) -> Result<(), std::sync::mpsc::RecvError> {
        loop {
            let message = self.receiver.recv()?;
            if message == waited_message {
                break;
            }
        }
        Ok(())
    }

    /// Waits for a specific message of type R to be received or times out after a specified duration.
    ///
    /// # Parameters
    /// - `waited_message`: The message to wait for.
    /// - `timeout`: The duration to wait before timing out.
    ///
    /// # Returns
    /// - `Ok(())` if the waited message is received before the timeout.
    /// - An error of type `std::sync::mpsc::RecvTimeoutError::Timeout` if the timeout occurs.
    /// - An error of type `std::sync::mpsc::RecvTimeoutError` if the receiving fails.
    pub fn wait_for_or_timeout(
        &self,
        waited_message: R,
        timeout: std::time::Duration,
    ) -> Result<(), std::sync::mpsc::RecvTimeoutError> {
        let start_time = std::time::Instant::now();

        let internal_timeout = timeout / 100;
        while start_time.elapsed() < timeout {
            // we map the internal_timeout to be very small to be able to quit as soon as the timeout is done
            // + having a dynamic internal_timeout is adding to the consistency
            match self.recv_timeout(internal_timeout) {
                Ok(message) => {
                    if message == waited_message {
                        return Ok(());
                    }
                }
                Err(err) => return Err(err),
            }
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout)
    }

    /// Short hand for [std::sync::mpsc::Sender::send]
    pub fn send(&self, t: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.sender.send(t)
    }

    /// Short hand for [std::sync::mpsc::Receiver::iter]
    pub fn iter(&self) -> std::sync::mpsc::Iter<'_, R> {
        self.receiver.iter()
    }

    /// Short hand for [std::sync::mpsc::Receiver::try_iter]
    pub fn try_iter(&self) -> std::sync::mpsc::TryIter<'_, R> {
        self.receiver.try_iter()
    }

    /// Short hand for [std::sync::mpsc::Receiver::recv]
    pub fn recv(&self) -> Result<R, std::sync::mpsc::RecvError> {
        self.receiver.recv()
    }

    /// Short hand for [std::sync::mpsc::Receiver::try_recv]
    pub fn try_recv(&self) -> Result<R, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Short hand for [std::sync::mpsc::Receiver::recv_timeout]
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<R, std::sync::mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}
