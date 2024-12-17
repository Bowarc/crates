#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("{0}")]
    Socket(#[from] crate::socket::SocketError),
    #[error("{0}")]
    Proxy(#[from] crate::proxy::ProxyError),
}
