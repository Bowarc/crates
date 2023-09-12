mod bps;
mod rtt;

#[derive(Default, Clone)]
pub struct NetworkStats<SRCW: crate::Message, SWCR: crate::Message> {
    rtt: rtt::Rtt,
    bps: bps::Bps,
    srcw: std::marker::PhantomData<SRCW>,
    swcr: std::marker::PhantomData<SWCR>,
}

impl<SRCW: crate::Message, SWCR: crate::Message> NetworkStats<SRCW, SWCR> {
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
