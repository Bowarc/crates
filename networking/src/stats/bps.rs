#[derive(Clone)]
pub struct Bps {
    total_sent: u64,
    total_received: u64,

    rolling_window: Vec<WindowEntry>,
    cfg: super::config::BpsConfig,
}
/*
    Do i warn when when the user is using bps while it's disabled ?
*/

#[derive(Clone)]
pub struct WindowEntry {
    time: std::time::Instant,
    bytes_sent: u64,
    bytes_received: u64,
}

impl Bps {
    pub fn new(cfg: super::config::BpsConfig) -> Self {
        let mut bps = Self {
            total_sent: 0,
            total_received: 0,
            rolling_window: Vec::new(),
            cfg,
        };
        bps.update();
        bps
    }
    fn enabled(&self) -> bool {
        self.cfg.enabled
    }

    pub fn update(&mut self) {
        let ten_seconds_ago = std::time::Instant::now() - std::time::Duration::from_secs(10);
        self.rolling_window.retain(|entry| {
            entry.time >= ten_seconds_ago && (entry.bytes_sent != 0 || entry.bytes_received != 0)
        });

        self.rolling_window.push(WindowEntry {
            time: std::time::Instant::now(),
            bytes_sent: 0,
            bytes_received: 0,
        });

        // println!("{} windows", self.rolling_window.len());
    }

    pub fn total_received(&self) -> u64 {
        self.total_received
    }
    pub fn total_sent(&self) -> u64 {
        self.total_sent
    }
    pub fn received_last_10_sec(&self) -> u64 {
        self.rolling_window
            .iter()
            .map(|entry| entry.bytes_received)
            .sum::<u64>()
    }
    pub fn bps_received_last_10_sec(&self) -> u64 {
        self.received_last_10_sec() / 10
    }
    pub fn sent_last_10_sec(&self) -> u64 {
        self.rolling_window
            .iter()
            .map(|entry| entry.bytes_sent)
            .sum::<u64>()
    }
    pub fn bps_sent_last_10_sec(&self) -> u64 {
        self.sent_last_10_sec() / 10
    }
    pub fn on_bytes_recv(&mut self, header: &crate::socket::Header) {
        self.total_received += header.size;
        self.rolling_window.last_mut().unwrap().bytes_received += header.size;
    }
    pub fn on_bytes_send(&mut self, header: &crate::socket::Header) {
        let byte_sent = header.size + crate::socket::HEADER_SIZE;
        self.total_sent += byte_sent;
        self.rolling_window.last_mut().unwrap().bytes_sent += byte_sent;
    }
}
