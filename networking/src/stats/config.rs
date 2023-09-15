#[derive(Default, Copy, Clone, Debug)]
pub struct BpsConfig {}

#[derive(Copy, Clone, Debug)]
pub struct RttConfig {
    pub ping_request_delay: std::time::Duration,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct StatConfig {}

impl Default for RttConfig {
    fn default() -> Self {
        Self {
            ping_request_delay: std::time::Duration::from_millis(1000),
        }
    }
}
