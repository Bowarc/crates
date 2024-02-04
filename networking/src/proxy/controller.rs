pub struct ProxyController<R: crate::Message, W: crate::Message> {
    stats: triple_buffer::Output<crate::NetworkStats<R, W>>,
    channel: threading::Channel<super::ProxyMessage<R>, W>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    connected: std::sync::Arc<std::sync::atomic::AtomicBool>,
    thread_handle: std::thread::JoinHandle<()>,
}

impl<R: crate::Message, W: crate::Message> ProxyController<R, W> {
    pub(crate) fn new(
        stats: triple_buffer::Output<crate::NetworkStats<R, W>>,
        channel: threading::Channel<super::ProxyMessage<R>, W>,
        running: std::sync::Arc<std::sync::atomic::AtomicBool>,
        connected: std::sync::Arc<std::sync::atomic::AtomicBool>,
        thread_handle: std::thread::JoinHandle<()>,
    ) -> ProxyController<R, W> {
        ProxyController {
            stats,
            channel,
            running,
            connected,
            thread_handle,
        }
    }

    pub fn send(&self, msg: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.channel.send(msg)
    }
    pub fn recv(&self) -> Result<super::ProxyMessage<R>, std::sync::mpsc::RecvError> {
        self.channel.recv()
    }
    pub fn try_recv(&self) -> Result<super::ProxyMessage<R>, std::sync::mpsc::TryRecvError>{
        self.channel.try_recv()
    }


    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn request_ping(&self) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.send(W::default_ping()) 
    }

    // Needs mut because it's updating before returning the data
    pub fn stats(&mut self) -> &crate::NetworkStats<R, W>{
        self.stats.read()
    }

    pub fn thread_handle(&self) -> &std::thread::JoinHandle<()>{
        &self.thread_handle
    }
}
