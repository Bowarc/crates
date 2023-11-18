#[derive(Copy, Clone, Debug)]
pub struct BpsConfig {
    pub enabled: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct RttConfig {
    pub enabled: bool,
    pub ping_request_delay: std::time::Duration,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct StatConfig {
    pub bps: BpsConfig,
    pub rtt: RttConfig,
}

// Readability
#[allow(clippy::derivable_impls)]
impl Default for BpsConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for RttConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            ping_request_delay: std::time::Duration::from_millis(1000),
        }
    }
}
