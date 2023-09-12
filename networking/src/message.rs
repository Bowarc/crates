pub trait Message:
    serde::Serialize
    + serde::de::DeserializeOwned
    + PartialEq
    + std::fmt::Debug
    + std::clone::Clone
    + std::marker::Send
{
    // For the network stats, it keeps track of the sent & received ping
    fn is_ping(&self) -> bool;
    // For the network stats, it keeps track of the sent & received pong
    fn is_pong(&self) -> bool;

    // Constructor for a default ping message (in case move than 1 of your messsage is used as ping)
    fn default_ping() -> Self;
    // Constructor for a default pong message (in case move than 1 of your messsage is used as ping)
    fn default_pong() -> Self;
}
