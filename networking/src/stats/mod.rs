mod bps;
mod rtt;

#[derive(Clone)]
pub struct NetworkStats<SRCW: crate::Message, SWCR: crate::Message> {
    rtt: rtt::Rtt,
    bps: bps::Bps,
    srcw: std::marker::PhantomData<SRCW>,
    swcr: std::marker::PhantomData<SWCR>,
}

impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn new() -> Self {
        Self {
            rtt: rtt::Rtt::default(),
            bps: bps::Bps::default(),
            srcw: std::marker::PhantomData,
            swcr: std::marker::PhantomData,
        }
    }
    pub fn update(
        &mut self,
        _channel: &mut threading::Channel<SWCR, SRCW>,
        socket: &mut crate::Socket<SRCW, SWCR>,
    ) {
        self.update_rtt(socket);
        self.bps.update();
    }

    // This can't be in rtt.update as you need the function on_msg_send and on_bytes_send
    fn update_rtt(&mut self, socket: &mut crate::Socket<SRCW, SWCR>) {
        if self.rtt.needs_ping() {
            let msg = SWCR::default_ping();
            self.on_msg_send(&msg);
            let header = socket.send(msg).unwrap();
            self.on_bytes_send(&header);
        }
    }

    pub fn on_msg_recv(&mut self, msg: &SRCW) {
        if msg.is_pong() {
            if let Some(stopwatch) = &self.rtt.ping_request_stopwatch {
                self.rtt.set(stopwatch.read());
                self.rtt.ping_request_stopwatch = None;
                self.rtt.last_pong = std::time::Instant::now();
            }
        }
    }
    pub fn on_bytes_recv(&mut self, header: &crate::socket::Header) {
        self.bps.on_bytes_recv(header)
    }

    pub fn on_msg_send(&mut self, msg: &SWCR) {
        if msg.is_ping() && self.rtt.ping_request_stopwatch.is_none() {
            self.rtt.ping_request_stopwatch = Some(time::Stopwatch::start_new())
        }
    }
    pub fn on_bytes_send(&mut self, header: &crate::socket::Header) {
        self.bps.on_bytes_send(header)
    }
}

// rtt
impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn set_rtt(&mut self, rtt: std::time::Duration) {
        self.rtt.set(rtt)
    }
    pub fn get_rtt(&self) -> std::time::Duration {
        self.rtt.get()
    }
}

//bps
impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
    pub fn total_received(&self) -> usize {
        self.bps.total_received()
    }
    pub fn total_sent(&self) -> usize {
        self.bps.total_sent()
    }
    pub fn received_last_10_sec(&self) -> usize {
        self.bps.received_last_10_sec()
    }
    pub fn bps_received_last_10_sec(&self) -> usize {
        self.bps.bps_received_last_10_sec()
    }
    pub fn sent_last_10_sec(&self) -> usize {
        self.bps.sent_last_10_sec()
    }
    pub fn bps_sent_last_10_sec(&self) -> usize {
        self.bps.bps_sent_last_10_sec()
    }
}

impl<SRCW: crate::Message, SWCR: crate::Message> Default for NetworkStats<SRCW, SWCR> {
    fn default() -> Self {
        Self::new()
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
                s: NetworkStats::<R, W>::new(),
            }
        }
    }
    let t = Testing::<ServerMessage, ClientMessage>::new();
}