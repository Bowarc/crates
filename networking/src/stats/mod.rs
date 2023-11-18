mod bps;
pub mod config;
mod rtt;

pub use config::StatConfig;

#[derive(Clone)]
pub struct NetworkStats<SRCW: crate::Message, SWCR: crate::Message> {
    bps_opt: Option<bps::Bps>,
    rtt_opt: Option<rtt::Rtt>,
    srcw: std::marker::PhantomData<SRCW>,
    swcr: std::marker::PhantomData<SWCR>,
    cfg: config::StatConfig,
}

impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn new(cfg: config::StatConfig) -> Self {
        Self {
            bps_opt: if cfg.bps.enabled {
                Some(bps::Bps::new(cfg.bps))
            } else {
                None
            },
            rtt_opt: if cfg.bps.enabled {
                Some(rtt::Rtt::new(cfg.rtt))
            } else {
                None
            },
            srcw: std::marker::PhantomData,
            swcr: std::marker::PhantomData,
            cfg,
        }
    }
    pub fn update(
        &mut self,
        _channel: &mut threading::Channel<SWCR, super::proxy::ProxyMessage<SRCW>>,
        socket: &mut crate::Socket<SRCW, SWCR>,
    ) -> Result<(), crate::socket::SocketError> {
        if self.cfg.rtt.enabled {
            self.update_rtt(socket)?;
        }

        if let Some(bps) = &mut self.bps_opt {
            bps.update();
        }
        Ok(())
    }

    // This can't be in rtt.update as you need the function on_msg_send and on_bytes_send
    fn update_rtt(
        &mut self,
        socket: &mut crate::Socket<SRCW, SWCR>,
    ) -> Result<(), crate::socket::SocketError> {
        let Some(rtt) = &mut self.rtt_opt else{
            return Ok(())
        };

        if rtt.needs_ping() {
            let msg = SWCR::default_ping();
            self.on_msg_send(&msg);
            let header = socket.send(msg)?;
            self.on_bytes_send(&header);
        }

        Ok(())
    }

    pub fn on_msg_recv(&mut self, msg: &SRCW, socket: &mut crate::Socket<SRCW, SWCR>) {
        if msg.is_ping() {
            let resp = SWCR::default_pong();
            self.on_msg_send(&resp);
            if let Ok(header) = socket.send(resp) {
                self.on_bytes_send(&header);
            } else {
                warn!("Could not send pong to {}", socket.remote_addr());
            }
        } else if msg.is_pong() {
            if let Some(rtt) = &mut self.rtt_opt {
                if let Some(stopwatch) = &rtt.ping_request_stopwatch {
                    rtt.set(stopwatch.read());
                    rtt.ping_request_stopwatch = None;
                    rtt.last_pong = std::time::Instant::now();
                }
            }
        }
    }
    pub fn on_bytes_recv(&mut self, header: &crate::socket::Header) {
        // we don't use if let else here because it's a general purpose function
        if let Some(bps) = &mut self.bps_opt {
            bps.on_bytes_recv(header)
        }
    }

    pub fn on_msg_send(&mut self, msg: &SWCR) {
        // we don't use if let else here because it's a general purpose function
        if let Some(rtt) = &mut self.rtt_opt {
            if msg.is_ping() && rtt.ping_request_stopwatch.is_none() {
                rtt.ping_request_stopwatch = Some(time::Stopwatch::start_new())
            }
        }
    }
    pub fn on_bytes_send(&mut self, header: &crate::socket::Header) {
        // we don't use if let else here because it's a general purpose function
        if let Some(bps) = &mut self.bps_opt {
            bps.on_bytes_send(header)
        }
    }
}

// rtt
impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn set_rtt(&mut self, duration: std::time::Duration) {
        if let Some(rtt) = &mut self.rtt_opt {
            rtt.set(duration)
        }
    }
    pub fn get_rtt(&self) -> std::time::Duration {
        self.rtt_opt
            .as_ref()
            .map(|rtt| rtt.get())
            .unwrap_or(std::time::Duration::ZERO)
    }
}

//bps
impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn total_received(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.total_received())
            .unwrap_or(0)
    }
    pub fn total_sent(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.total_sent())
            .unwrap_or(0)
    }
    pub fn received_last_10_sec(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.received_last_10_sec())
            .unwrap_or(0)
    }
    pub fn bps_received_last_10_sec(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.bps_received_last_10_sec())
            .unwrap_or(0)
    }
    pub fn sent_last_10_sec(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.sent_last_10_sec())
            .unwrap_or(0)
    }
    pub fn bps_sent_last_10_sec(&self) -> usize {
        self.bps_opt
            .as_ref()
            .map(|bps| bps.bps_sent_last_10_sec())
            .unwrap_or(0)
    }
}

impl<SRCW: crate::Message, SWCR: crate::Message> Default for NetworkStats<SRCW, SWCR> {
    fn default() -> Self {
        Self {
            bps_opt: None,
            rtt_opt: None,
            srcw: std::marker::PhantomData,
            swcr: std::marker::PhantomData,
            cfg: config::StatConfig::default(),
        }
    }
}

#[test]
fn testing() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
    pub enum ClientMessage {
        Text(String),
        Ping,
        Pong,
    }
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
    pub enum ServerMessage {
        Text(String),
        Ping,
        Pong,
    }

    impl crate::Message for ClientMessage {
        fn is_ping(&self) -> bool {
            matches!(self, Self::Ping)
        }
        fn is_pong(&self) -> bool {
            matches!(self, Self::Pong)
        }

        fn default_ping() -> Self {
            Self::Ping
        }
        fn default_pong() -> Self {
            Self::Pong
        }
    }

    impl crate::Message for ServerMessage {
        fn is_ping(&self) -> bool {
            matches!(self, Self::Ping)
        }
        fn is_pong(&self) -> bool {
            matches!(self, Self::Pong)
        }

        fn default_ping() -> Self {
            Self::Ping
        }
        fn default_pong() -> Self {
            Self::Pong
        }
    }

    pub struct Testing<R: crate::Message, W: crate::Message> {
        s: NetworkStats<R, W>,
    }

    impl<R: crate::Message, W: crate::Message> Testing<R, W> {
        fn new() -> Self {
            Self {
                s: NetworkStats::<R, W>::new(StatConfig::default()),
            }
        }
    }
    let _t = Testing::<ServerMessage, ClientMessage>::new();
}
