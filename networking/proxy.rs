pub struct ClientProxy {
    server: shared::networking::Socket<
        shared::networking::ServerMessage,
        shared::networking::ClientMessage,
    >,
    client: shared::threading::Channel<
        shared::networking::ClientMessage,
        shared::networking::ServerMessage,
    >,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Input<super::NetworkStats>,
}

impl ClientProxy {
    pub fn new(
        stream: std::net::TcpStream,
        client: shared::threading::Channel<
            shared::networking::ClientMessage,
            shared::networking::ServerMessage,
        >,
        running: std::sync::Arc<std::sync::atomic::AtomicBool>,
        stats: triple_buffer::Input<super::NetworkStats>,
    ) -> Self {
        Self {
            server: shared::networking::Socket::<
                shared::networking::ServerMessage,
                shared::networking::ClientMessage,
            >::new(stream),
            client,
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
            stats.update(&mut self.client, &mut self.server);

            if let Err(e) = self.handle_client(&mut stats) {
                warn!("Proxy encountered an error while handling client {e:?}");
                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed);
            }

            if let Err(e) = self.handle_server(&mut stats) {
                warn!("Proxy encountered an error while handling server {e:?}");
                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed);
            }

            self.stats.write(stats);

            loop_helper.loop_sleep();
        }

        self.server.shutdown();

        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        debug!(
            "Client proxy for ({}) has exited",
            self.server.remote_addr()
        );
    }
    fn handle_client(
        &mut self,
        stats: &mut super::NetworkStats,
    ) -> Result<(), super::NetworkError> {
        // here you receive the message sent by the client

        if let Ok(msg) = self.client.try_recv() {
            stats.on_msg_send(&msg);
            match self.server.send(msg) {
                Ok(header) => {
                    // Do something with the number of bytes sent in the stats
                    stats.on_bytes_send(&header);
                }
                Err(e) => {
                    error!(
                        "Proxy encountered an error while forwarding a message to the server: {e:?}"
                    );
                    Err(e).map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?
                }
            }
        }

        Ok(())
    }

    fn handle_server(
        &mut self,
        stats: &mut super::NetworkStats,
    ) -> Result<(), super::NetworkError> {
        // here you receive message sent by the client
        match self.server.try_recv() {
            Ok((header, msg)) => {
                stats.on_msg_recv(&msg);
                stats.on_bytes_recv(&header);

                self.client
                    .send(msg)
                    .map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?;
            }
            Err(e) => {
                // Check if the error is from the fact that the proxy's stream is non_bocking
                let is_would_block =
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        io_e.kind() == std::io::ErrorKind::WouldBlock
                    } else {
                        // matches!(e, shared::networking::SocketError::WouldBlock)

                        false
                    };

                //if it's not from that.. it's a real error
                if !is_would_block {
                    self.running
                        .store(false, std::sync::atomic::Ordering::Relaxed);

                    // The error might just be that the server disconnected
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        if io_e.kind() == std::io::ErrorKind::ConnectionReset {
                            warn!("Server {ip} disconnected", ip = self.server.remote_addr());
                        }
                    } else {
                        error!(
                            "Error while listening server {}, aborting: {e}",
                            self.server.remote_addr()
                        );
                    }

                    Err(e)?
                }
            }
        }

        Ok(())
    }
}
