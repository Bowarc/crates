#[cfg_attr(feature = "bevy", derive(bevy::ecs::prelude::Resource))]
pub struct LoggerThreadHandle {
    sender: std::sync::mpsc::Sender<crate::Message>,
    inner: Option<std::thread::JoinHandle<()>>,
}

impl LoggerThreadHandle {
    pub(crate) fn new(
        sender: std::sync::mpsc::Sender<crate::Message>,
        inner: std::thread::JoinHandle<()>,
    ) -> Self {
        Self { sender, inner: Some(inner) }
    }
}

impl Drop for LoggerThreadHandle {
    fn drop(&mut self) {
        if let Err(e) = self.sender.send(crate::Message::Exit){
            eprintln!("[ERROR] Failed to close logger thread due to: {e}");
            return;
        }
        let Some(handle) = self.inner.take() else {
            eprintln!("[ERROR] Logger thread was already dropped once");
            return;
        };

        if let Err(e) = handle.join(){
            eprintln!("[ERROR] Failed to join the logger thread due to: {e:?}");
        }
    }
}
