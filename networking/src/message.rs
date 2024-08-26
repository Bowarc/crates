pub trait Message:
    serde::Serialize
    + serde::de::DeserializeOwned
    + PartialEq
    + std::fmt::Debug
    + std::clone::Clone
    + std::marker::Send
{
    /// Used to make disconnection faster
    fn is_exit(&self) -> bool {
        panic!("The networing::Message::is_exit method is not implemented for {self:?}")
    }
    // For the network stats, it keeps track of the sent & received ping
    fn is_ping(&self) -> bool {
        panic!("The networing::Message::is_ping method is not implemented for {self:?}")
    }
    // For the network stats, it keeps track of the sent & received pong
    fn is_pong(&self) -> bool {
        panic!("The networing::Message::is_pong method is not implemented for {self:?}")
    }

    // Constructor for a default exit message (The variant given by this method has to return true on Message::is_exit)
    fn default_exit() -> Self {
        panic!(
            "The networing::Message::default_exit method is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
    // Constructor for a default ping message (The variant given by this method has to return true on Message::is_ping)
    fn default_ping() -> Self {
        panic!(
            "The networing::Message::default_ping method is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
    // Constructor for a default pong message (The variant given by this method has to return true on Message::is_pong)
    fn default_pong() -> Self {
        panic!(
            "The networing::Message::default_pong method is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
}
