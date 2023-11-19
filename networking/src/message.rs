pub trait Message:
    serde::Serialize
    + serde::de::DeserializeOwned
    + PartialEq
    + std::fmt::Debug
    + std::clone::Clone
    + std::marker::Send
{
    // For the network stats, it keeps track of the sent & received ping
    fn is_ping(&self) -> bool {
        panic!("The networing::Message::is_ping method is not implemented for {self:?}")
    }
    // For the network stats, it keeps track of the sent & received pong
    fn is_pong(&self) -> bool {
        panic!("The networing::Message::is_pong method is not implemented for {self:?}")
    }

    // Constructor for a default ping message (in case more than 1 of your messsage is used as ping)
    fn default_ping() -> Self {
        panic!(
            "The networing::Message::default_ping method is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
    // Constructor for a default pong message (in case more than 1 of your messsage is used as ping)
    fn default_pong() -> Self {
        panic!(
            "The networing::Message::default_pong method is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
}
