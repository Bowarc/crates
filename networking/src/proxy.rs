// as args, do i say that Read is the local or distant
// Socket Read Channel Write
// Socket Write Channel Read
pub struct Proxy<SRCW: crate::Message, SWCR: crate::Message> {
    socket: crate::Socket<SRCW, SWCR>,
    channel: threading::Channel<SWCR, SRCW>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Input<super::NetworkStats<SRCW, SWCR>>,
}

#[derive(thiserror::Error, Debug)]
pub enum ProxyError {
    #[error("{0}")]
    ChannelSend(String),
}

impl<SRCW: crate::Message, SWCR: crate::Message> Proxy<SRCW, SWCR> {
    pub fn new(
        stream: std::net::TcpStream,
        channel: threading::Channel<SWCR, SRCW>,
        running: std::sync::Arc<std::sync::atomic::AtomicBool>,
        stats: triple_buffer::Input<super::NetworkStats<SRCW, SWCR>>,
    ) -> Self {
        Self {
            socket: crate::Socket::<SRCW, SWCR>::new(stream),
            channel,
            running,
            stats,
        }
    }

    pub fn run(&mut self) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(10000.);

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            loop_helper.loop_start();

            let mut stats = self.stats.read().clone();
            if let Err(e) = stats.update(&mut self.channel, &mut self.socket) {
                warn!(
                    "Stats update encountered an error: {e}, stopping proxy for {}",
                    self.socket.remote_addr()
                );
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

        self.socket.shutdown();

        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        debug!("Proxy for ({}) has exited", self.socket.remote_addr());
    }
    fn handle_channel(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), super::NetworkError> {
        // here you receive the message sent by the channel

        if let Ok(msg) = self.channel.try_recv() {
            stats.on_msg_send(&msg);
            match self.socket.send(msg) {
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

    fn handle_socket(
        &mut self,
        stats: &mut super::NetworkStats<SRCW, SWCR>,
    ) -> Result<(), super::NetworkError> {
        // here you receive message sent by the client
        match self.socket.try_recv() {
            Ok((header, msg)) => {
                stats.on_msg_recv(&msg);
                stats.on_bytes_recv(&header);

                self.channel
                    .send(msg)
                    .map_err(|e| ProxyError::ChannelSend(format!("{e}")))?
                // .map_err(|e| super::NetworkError::Proxy(format!("{e:?}")))?;
            }
            Err(e) => {
                // Check if the error is from the fact that the proxy's stream is non_bocking
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
                            warn!("socket {ip} disconnected", ip = self.socket.remote_addr());
                        }
                    } else {
                        error!(
                            "Error while listening socket {}, aborting: {e}",
                            self.socket.remote_addr()
                        );
                    }

                    Err(e)?
                }
            }
        }

        Ok(())
    }
}
