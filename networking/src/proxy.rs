// as args, do i say that Read is the local or distant
// Socket Read Channel Write
// Socket Write Channel Read
pub struct Proxy<SRCW: crate::Message, SWCR: crate::Message> {
    addr: std::net::SocketAddr,
    cfg: ProxyConfig,
    socket_opt: Option<crate::Socket<SRCW, SWCR>>,
    channel: threading::Channel<SWCR, ProxyMessage<SRCW>>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Input<super::NetworkStats<SRCW, SWCR>>,
}

#[derive(Copy, Clone, Debug)]
pub struct ProxyConfig {
    pub addr: std::net::SocketAddr,
    pub run_tps: usize,
    pub stat_cfg: crate::stats::StatConfig,
}

pub struct ProxyOutput<SRCW: crate::Message, SWCR: crate::Message> {
    pub stats: triple_buffer::Output<super::NetworkStats<SRCW, SWCR>>,
    pub channel: threading::Channel<ProxyMessage<SRCW>, SWCR>,
    pub running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub thread_handle: std::thread::JoinHandle<()>,
}

#[derive(PartialEq, Debug)]
pub enum ProxyMessage<T: crate::Message> {
    Forward(T),
    ConnectionResetError,
    Exit { error: bool },
}

#[derive(thiserror::Error, Debug)]
pub enum ProxyError {
    #[error("{0}")]
    ChannelSend(String),
    #[error("Proxy is disconnected")]
    Disconnected,
}

impl<SRCW: crate::Message + 'static, SWCR: crate::Message + 'static> Proxy<SRCW, SWCR> {
    pub fn start_new(cfg: ProxyConfig) -> ProxyOutput<SRCW, SWCR> {
        let (proxy_channel, main_channel) =
            threading::Channel::<ProxyMessage<SRCW>, SWCR>::new_pair();

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let (stats_in, stats_out) =
            triple_buffer::TripleBuffer::new(&crate::NetworkStats::new(cfg.stat_cfg)).split();

        let proxy = Proxy::<SRCW, SWCR> {
            addr: cfg.addr,
            cfg,
            socket_opt: None,
            channel: proxy_channel,
            running: running.clone(),
            stats: stats_in,
        };

        let thread_handle = std::thread::spawn(move || proxy.run());

        ProxyOutput {
            stats: stats_out,
            channel: main_channel,
            running,
            thread_handle,
        }
    }

    fn try_connect(&mut self) {
        if let Ok(stream) = std::net::TcpStream::connect(self.addr) {
            self.socket_opt = Some(crate::Socket::new(stream));
            self.set_running(true)
        } else {
            self.set_running(false)
        }
    }
    fn set_running(&mut self, val: bool) {
        self.running
            .store(val, std::sync::atomic::Ordering::Relaxed)
    }

    fn reset_connection(&mut self) {
        self.socket_opt = None;
        if let Err(e) = self.channel.send(ProxyMessage::ConnectionResetError) {
            error!(
                "Could not send {:?} message to main thread, {e}",
                ProxyMessage::ConnectionResetError::<SRCW>
            )
        }
    }
    fn run(mut self) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(self.cfg.run_tps as f64);

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            loop_helper.loop_start();

            let mut stats = self.stats.read().clone();

            let Some(socket) = &mut self.socket_opt else{
                self.try_connect();
                continue;
            };

            if let Err(e) = stats.update(&mut self.channel, socket) {
                warn!("Stats update encountered an error: {e}, stopping proxy",);
                break;
            }

            if let Err(e) = self.handle_channel(&mut stats) {
                warn!("Proxy encountered an error while handling channel {e:?}");
                break;
            }

            if let Err(e) = self.handle_socket(&mut stats) {
                warn!("Proxy encountered an error while handling socket {e:?}");
                break;
            }

            self.stats.write(stats);

            loop_helper.loop_sleep();
        }

        if let Err(e) = self.channel.send(ProxyMessage::Exit { error: false }) {
            error!("Could not send exit message to main thread: {e}")
        }

        self.set_running(false);

        if let Some(socket) = self.socket_opt {
            socket.shutdown();
        }

        // Give a bit of time to everything to synchronise, and exit cleanly
        spin_sleep::sleep(std::time::Duration::from_secs(1));

        debug!("Proxy for ({}) has exited", self.addr);
    }
    /// here you receive the message sent by the channel
    fn handle_channel(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), super::NetworkError> {
        let Some(socket) = &mut self.socket_opt else{
            return Err(super::NetworkError::Proxy(ProxyError::Disconnected));
        };

        if let Ok(msg) = self.channel.try_recv() {
            stats.on_msg_send(&msg);
            match socket.send(msg) {
                Ok(header) => {
                    // Do something with the number of bytes sent in the stats
                    stats.on_bytes_send(&header);
                }
                Err(e) => {
                    error!(
                        "Proxy encountered an error while forwarding a message to the server: {e:?}"
                    );
                    Err(e).map_err(|e| ProxyError::ChannelSend(format!("{e:?}")))?
                }
            }
        }

        Ok(())
    }

    /// here you receive message sent by the client
    fn handle_socket(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), super::NetworkError> {
        let Some(socket) = &mut self.socket_opt else{
            return Err(super::NetworkError::Proxy(ProxyError::Disconnected));
        };

        match socket.try_recv() {
            Ok((header, msg)) => {
                stats.on_msg_recv(&msg, socket);
                stats.on_bytes_recv(&header);

                self.channel
                    .send(ProxyMessage::Forward(msg))
                    .map_err(|e| ProxyError::ChannelSend(format!("{e}")))?
                // .map_err(|e| super::NetworkError::Proxy(format!("{e:?}")))?;
            }
            Err(e) => {
                // Check if the error is from the fact that the proxy's stream is non_bocking
                // Here i could remove that 'useless' mem allocation but it would make the code not readable
                let is_would_block = if let crate::socket::SocketError::StreamRead(ref io_e) = e {
                    io_e.kind() == std::io::ErrorKind::WouldBlock
                } else {
                    // matches!(e, shared::networking::SocketError::WouldBlock)

                    false
                };

                //if it's not from that.. it's a real error
                if !is_would_block {
                    self.running
                        .store(false, std::sync::atomic::Ordering::Relaxed);

                    // The error might just be that the socket disconnected
                    if let crate::socket::SocketError::StreamRead(ref io_e) = e {
                        if io_e.kind() == std::io::ErrorKind::ConnectionReset {
                            warn!("socket {ip} disconnected", ip = socket.remote_addr());
                        }
                    } else {
                        error!(
                            "Error while listening socket {}, aborting: {e}",
                            socket.remote_addr()
                        );
                    }
                    // Reset the connection
                    self.socket_opt = None;
                    Err(e)?
                }
            }
        }

        Ok(())
    }
}
