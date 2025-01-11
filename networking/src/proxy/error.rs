#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ProxyError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    ChannelSend(String),
    #[error("{0}")]
    ChannelRecv(String),

    #[error("{0}")]
    SocketSend(String),
    #[error("{0}")]
    SocketRecv(String),

    #[error("Proxy is disconnected")]
    Disconnected,
}
