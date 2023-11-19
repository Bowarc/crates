#[test]
fn socket() {
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    enum Message {
        Text(String),
        // ..
    }
    impl networking::Message for Message {
        // Methods of this trait are used for stat calculation (Used by networking::Proxy)
    }

    // Assuming there is a std::net::TcpListener at this address
    let stream = std::net::TcpStream::connect("127.0.0.1:42069").unwrap();
    // Read and Write can be different types, for client vs server msg
    let mut socket: networking::Socket<Message, Message> = networking::Socket::new(stream);

    // This is non-blocking
    // The header holds the byte size of the received message, probably not usefull to most
    let recv_res: Result<(networking::socket::Header, Message), networking::socket::SocketError> =
        socket.try_recv();

    if let Ok((_header, message)) = recv_res {
        println!(
            "Received {message:?} from {remote_addr}",
            remote_addr = socket.remote_addr()
        )
    }

    // The header holds the byte size of the sent message, probably not usefull to most
    let _sent_res: Result<networking::socket::Header, networking::socket::SocketError> =
        socket.send(Message::Text(String::from("Hellow")));
}
