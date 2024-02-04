#[derive(Copy, Clone, Debug)]
pub struct ProxyConfig {
    pub addr: std::net::SocketAddr,
    pub run_tps: u64,
    pub stat_cfg: crate::stats::StatConfig,
    // https://github.com/Bowarc/Crates/issues/8
    pub keep_msg_while_disconnected: bool,
    pub auto_reconnect: bool,
}
