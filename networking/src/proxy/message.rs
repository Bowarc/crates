#[derive(PartialEq, Debug)]
pub enum ProxyMessage<T: crate::Message> {
    Forward(T),
    ConnectionResetError,
    Exit,
}
