pub const HEADER_SIZE: usize = std::mem::size_of::<Header>();

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
// ofc don't use type that can change size (such as Vec) so the size of the struct stays the same as the constant
pub struct Header {
    pub size: usize,
}

// I don't like how streams work so i'll make a simple socket-like, packet-based struct wrapper
pub struct Socket<R, W> {
    stream: std::net::TcpStream,
    read_type: std::marker::PhantomData<R>,
    write_type: std::marker::PhantomData<W>,
    last_header: Option<Header>,
}

#[derive(thiserror::Error, Debug)]
pub enum SocketError {
    #[error("This should not be used outside tests")]
    TestError,
    #[error("Error when serializing: {0}")]
    Serialization(bincode::Error),
    #[error("Error when deserializing: {0}")]
    Deserialization(bincode::Error),
    #[error("Error when writing to stream: {0}")]
    StreamWrite(std::io::Error),
    #[error("Error when reading the stream: {0}")]
    StreamRead(std::io::Error),
    // #[error("Error when peeking into stream: {0}")]
    // StreamPeek(std::io::Error),
    // #[error("Still waiting for more data")]
    // WouldBlock,
}

impl Header {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl<R: crate::Message, W: crate::Message> Socket<R, W> {
    pub fn new(stream: std::net::TcpStream) -> Self {
        Self {
            stream,
            read_type: std::marker::PhantomData,
            write_type: std::marker::PhantomData,
            last_header: None,
        }
    }
    pub fn send(&mut self, message: W) -> Result<Header, SocketError> {
        use std::io::Write as _;

        let message_bytes = bincode::serialize(&message).map_err(SocketError::Serialization)?;

        let header = Header::new(message_bytes.len());

        let header_bytes = bincode::serialize(&header).map_err(SocketError::Serialization)?;

        // idk if panicking is a good idea
        // assert_eq!(header_bytes.len(), HEADER_SIZE);
        if header_bytes.len() != HEADER_SIZE {
            return Err(SocketError::Serialization(Box::new(bincode::ErrorKind::Custom(format!("The length of the serialized header is not equal to the HEADER_SIZE constant ({HEADER_SIZE})"))),));
        }

        self.stream
            .write_all(&header_bytes)
            .map_err(SocketError::StreamWrite)?;
        trace!("Sending {:?}:  {:?}", header, header_bytes);

        self.stream
            .write_all(&message_bytes)
            .map_err(SocketError::StreamWrite)?;
        trace!("Sending {:?}:  {:?}", message, message_bytes);

        Ok(header)
    }
    pub fn try_recv(&mut self) -> Result<(Header, R), SocketError> {
        let header = match self.last_header {
            Some(header) => {
                debug!("Using saved header: {header:?}");
                header
            }
            None => {
                let header = self.try_get::<Header>(HEADER_SIZE)?;

                self.last_header = Some(header);
                header
            }
        };

        let message = self.try_get::<R>(header.size)?;

        self.last_header = None;

        Ok((header, message))
    }

    fn try_get<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &mut self,
        target_size: usize,
    ) -> Result<T, SocketError> {
        use std::io::Read as _;
        let mut peek_buffer = vec![0; target_size];

        let read_len = self
            .stream
            .peek(&mut peek_buffer)
            .map_err(SocketError::StreamRead)?;

        if read_len != 0 {
            trace!(
                "Peeking steam, looking for {} bytes.. Done, found {} bytes",
                target_size,
                read_len
            );
        }

        if read_len != target_size {
            if read_len != 0 {
                warn!("Read {} but was waiting for {}", read_len, target_size);
            }
            return Err(SocketError::StreamRead(std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                "",
            )));
        }

        let mut message_buffer = vec![0; target_size];

        self.stream
            .read_exact(&mut message_buffer)
            .map_err(SocketError::StreamRead)?;

        let message: T =
            bincode::deserialize(&message_buffer).map_err(SocketError::Deserialization)?;
        trace!("Deserializing message.. Done, {message:?}");

        Ok(message)
    }

    pub fn local_addr(&self) -> std::net::SocketAddr {
        self.stream.local_addr().unwrap()
    }

    pub fn remote_addr(&self) -> std::net::SocketAddr {
        self.stream.peer_addr().unwrap()
    }
    pub fn shutdown(&self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
