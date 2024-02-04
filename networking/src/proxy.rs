mod config;
mod message;
mod error;
mod controller;


pub use config::ProxyConfig;
pub use message::ProxyMessage;
pub use error::ProxyError;
pub use controller::ProxyController;

// as args, do i say that Read is the local or distant
// Socket Read Channel Write
// Socket Write Channel Read
pub struct Proxy<SRCW: crate::Message, SWCR: crate::Message> {
    cfg: ProxyConfig,
    socket_opt: Option<crate::Socket<SRCW, SWCR>>,
    channel: threading::Channel<SWCR, ProxyMessage<SRCW>>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    connected: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Input<super::NetworkStats<SRCW, SWCR>>,
}




impl<SRCW: crate::Message + 'static, SWCR: crate::Message + 'static> Proxy<SRCW, SWCR> {
    pub fn start_new(
        cfg: ProxyConfig,
        stream_opt: Option<std::net::TcpStream>,
    ) -> ProxyController<SRCW, SWCR> {
        let (proxy_channel, main_channel) =
            threading::Channel::<ProxyMessage<SRCW>, SWCR>::new_pair();

        let socket_opt = stream_opt.map(crate::Socket::new);

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let connected =
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(socket_opt.is_some()));

        let (stats_in, stats_out) =
            triple_buffer::TripleBuffer::new(&crate::NetworkStats::new(cfg.stat_cfg)).split();

        let proxy = Proxy::<SRCW, SWCR> {
            cfg,
            socket_opt,
            channel: proxy_channel,
            running: running.clone(),
            connected: connected.clone(),
            stats: stats_in,
        };

        let thread_handle = std::thread::spawn(move || proxy.run());

        ProxyController {
            stats: stats_out,
            channel: main_channel,
            running,
            connected,
            thread_handle,
        }
    }

    fn try_connect(&mut self) -> Result<(), ProxyError>{
        trace!("Trying to reconnect");
        match std::net::TcpStream::connect(self.cfg.addr) {
            Ok(stream) => {
                if let Err(e) = stream.set_nonblocking(true) {
                    error!("Could not set the created stream to non-blocking: {e}");
                    return Err(ProxyError::Config(format!("Could not set stream to non-blocking due to: {e}")));
                }
                self.socket_opt = Some(crate::Socket::new(stream));
                self.set_connected(true);
                if !self.cfg.keep_msg_while_disconnected {
                    while let Ok(value) = self.channel.try_recv() {
                        drop(value)
                    }
                }
            }
            Err(e) => {
                error!("Could not connect: {e}");
                self.set_connected(false);
            }
        }
        Ok(())
    }
    fn set_connected(&mut self, val: bool) {
        self.connected
            .store(val, std::sync::atomic::Ordering::Relaxed)
    }

    fn set_running(&mut self, val: bool) {
        self.running
            .store(val, std::sync::atomic::Ordering::Relaxed)
    }

    fn reset_connection(&mut self) {
        self.set_connected(false);
        self.socket_opt = None;
        if let Err(e) = self.channel.send(ProxyMessage::ConnectionResetError) {
            error!(
                "Could not send {:?} message to main thread, {e}",
                ProxyMessage::ConnectionResetError::<SRCW>
            );
            self.handle_error(ProxyError::ChannelSend(e.to_string()));
        }
    }

    fn handle_error(&mut self, error: ProxyError){
        match error{
            ProxyError::Config(e) => {
                warn!("{e}");
                self.set_running(false);
            }
            ProxyError::ChannelSend(e) => {
                error!("{e}");
                self.set_running(false);
            }
            ProxyError::ChannelRecv(e) => {
                error!("{e}");
                self.set_running(false);
            }
            ProxyError::SocketSend(e) => {
                error!("{e}");
                if self.cfg.auto_reconnect {
                    self.reset_connection();
                }else{
                    self.set_running(false);   
                }
            }
            ProxyError::SocketRecv(e) => {
                error!("{e}");
                if self.cfg.auto_reconnect {
                    self.reset_connection();
                }else{
                    self.set_running(false);   
                }
            }
            ProxyError::Disconnected => {}
        }

    }

    fn run(mut self) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(self.cfg.run_tps as f64);

        if self.socket_opt.is_none() {
            if let Err(e) = self.try_connect(){
                self.handle_error(e)
            }
        }

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            loop_helper.loop_start();

            let mut stats = self.stats.read().clone();

            let Some(socket) = &mut self.socket_opt else{
                if self.cfg.auto_reconnect{
                    if let Err(e) = self.try_connect(){
                        self.handle_error(e);
                    }
                    continue;
                }
                else{
                    break;
                }
            };

            if let Err(e) = stats.update(&mut self.channel, socket) {
                self.handle_error(e)
            }

            if let Err(e) = self.handle_local(&mut stats) {
                self.handle_error(e);
                continue;
            }

            if let Err(e) = self.handle_distant(&mut stats) {
                self.handle_error(e);
                continue;
            }


            self.stats.write(stats);

            loop_helper.loop_sleep();
        }

        if let Err(e) = self.channel.send(ProxyMessage::Exit) {
            error!("Could not send exit message to main thread: {e}")
        }

        self.set_running(false);
        self.set_connected(false);

        if let Some(socket) = self.socket_opt {
            socket.shutdown();
        }

        // Give a bit of time to everything to synchronise, and exit cleanly
        spin_sleep::sleep(std::time::Duration::from_secs(1));

        debug!("Proxy for ({}) has exited", self.cfg.addr);
    }
    /// here you receive the message sent by the channel
    fn handle_local(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), ProxyError> {
        let Some(socket) = &mut self.socket_opt else{
            return Err(ProxyError::Disconnected);
        };

        match self.channel.try_recv(){
            Ok(local_msg) => {
                stats.on_msg_send(&local_msg);
                match socket.send(local_msg) {
                    Ok(header) => {
                        // Do something with the number of bytes sent in the stats
                        stats.on_bytes_send(&header);
                    }
                    Err(e) => {
                        error!(
                            "Proxy encountered an error while forwarding a message to the server: {e:?}"
                        );
                        return Err(e).map_err(|e| ProxyError::SocketSend(format!("{e:?}")))?
                    }
                }
            },
            Err(e) => {
                match e{
                    std::sync::mpsc::TryRecvError::Empty => (),
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        error!("Proxy encountered an error while listening local channel: {e:?}");
                        return Err(ProxyError::ChannelRecv(e.to_string()))
                    },
                }
            },
        }

        Ok(())
    }

    /// here you receive message sent by the socket
    fn handle_distant(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), ProxyError> {
        let Some(socket) = &mut self.socket_opt else{
            return Err(ProxyError::Disconnected);
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
                    // The error might just be that the socket disconnected
                    if let crate::socket::SocketError::StreamRead(ref io_e) = e {
                        if io_e.kind() == std::io::ErrorKind::ConnectionReset {
                            warn!("socket {addr} disconnected", addr = self.cfg.addr);
                        }
                    } else {
                        error!(
                            "Error while listening socket {}: {e}",
                            socket.remote_addr()
                        );
                    }
                    self.reset_connection();
                    return Err(ProxyError::SocketRecv(e.to_string()))
                }
            }
        }

        Ok(())
    }
}
