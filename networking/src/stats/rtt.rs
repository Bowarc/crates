#[derive(Clone)]
pub struct Rtt {
    pub latest_rtt: std::time::Duration,
    pub ping_request_stopwatch: Option<time::Stopwatch>,
    pub last_pong: std::time::Instant,
}

impl Rtt {
    pub fn needs_ping(&self) -> bool {
        // self.last_pong.elapsed() > std::time::Duration::from_secs_f32(0.1)
        //     && self.ping_request_stopwatch.is_none()
        true
    }

    pub fn set(&mut self, rtt: std::time::Duration) {
        self.latest_rtt = rtt
    }
    pub fn get(&self) -> std::time::Duration {
        self.latest_rtt
    }
}

impl Default for Rtt {
    fn default() -> Self {
        Self {
            latest_rtt: std::time::Duration::ZERO,
            ping_request_stopwatch: None,
            last_pong: std::time::Instant::now(),
        }
    }
}
