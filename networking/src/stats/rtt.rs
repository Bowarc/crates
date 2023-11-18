#[derive(Clone)]
pub struct Rtt {
    pub latest_rtt: std::time::Duration,
    pub ping_request_stopwatch: Option<time::Stopwatch>,
    pub last_pong: std::time::Instant,
    cfg: super::config::RttConfig,
}

impl Rtt {
    pub fn new(cfg: super::config::RttConfig) -> Self {
        Self {
            latest_rtt: std::time::Duration::ZERO,
            ping_request_stopwatch: None,
            last_pong: std::time::Instant::now(),
            cfg,
        }
    }

    pub fn needs_ping(&self) -> bool {
        if !self.cfg.enabled {
            return false;
        }
        self.last_pong.elapsed() > self.cfg.ping_request_delay
            && self.ping_request_stopwatch.is_none()
    }

    pub fn set(&mut self, rtt: std::time::Duration) {
        self.latest_rtt = rtt
    }
    pub fn get(&self) -> std::time::Duration {
        self.latest_rtt
    }
}
