## Simple Tcp connections

### Features
- Socket: A socket-style wrapper arround rust's `std::net::TcpStream` with generics Read and Write types
- Proxy: A Socket that lives in another thread to remove the (de)serialisation overhead
- Stats: A structure used by Proxy that allows you to have basic stats about the proxy (round trip time (ping) and the number of bytes exchanged (Overall or over the last 1/10 seconds), more later)


#### Use example for Socket:

cargo.toml
```toml
[dependencies]
networking = {git = "https://github.com/Bowarc/Crates.git", package = "networking"}
serde = { version = "1.0.188", features = ["derive"] }
``` 
main.rs
```rust
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

if let Ok((_header, message)) = recv_res{
    println!("Received {message:?} from {remote_addr}", remote_addr = socket.remote_addr())
}

// The header holds the byte size of the sent message, probably not usefull to most
let _sent_res: Result<networking::socket::Header, networking::socket::SocketError> =
    socket.send(Message::Text(String::from("Hellow")));
``` 

### Use example for Proxy:

cargo.toml
```toml
[dependencies]
logger = {git = "https://github.com/Bowarc/Crates.git", package = "networking"}
serde = { version = "1.0.188", features = ["derive"] }
``` 

main.rs
```rust
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Message {
    Text(String),
    // ..
}

impl networking::Message for Message {
    /*
    Methods of this trait are used for stat calculation
    if you want to enable the stats, don't forget to impl thoses

    IMPORTANT
        When RTT stat is disabled in proxy config, the proxy will not anwser to any ping
        calls from the remote, therefore their rtt calculation will not work.
    */
}

let addr = std::net::SocketAddr::from_str("127.0.0.1:42069").unwrap();
let proxy_cfg = networking::proxy::ProxyConfig {
    // The address where the proxy has to connect
    addr,
    // The target Tick Per Second of the Proxy's internal loop
    run_tps: 10, // 10 updates per second
    // Everything is disabled by default
    stat_cfg: Default::default(),
    // This option set to true will stores the msg that you send to the proxy while the proxy is disconnected
    // And will send them as soon as it reconnects
    // keep this to false unless you know what you are doing
    keep_msg_while_disconnected: false,
    // Auto reconnect to the given address
    auto_reconnect: false,
};
/*
Note:
    The proxy will not send you the raw message that it received
    It sends you a ProxyMessage<R> // R being the receive type that you've set
    It allows you to be informed when the proxy's connection stopped or when the proxy stops

    You also have two std::sync::Arc<std::sync::atomic::AtomicBool>,
    one for the connection, the other to check if the proxy is running
*/
// Generics: What you recv, what you send
let proxy_output: networking::proxy::ProxyOutput<Message, Message> =
    networking::Proxy::start_new(proxy_cfg, None);

proxy_output
    .channel
    .send(Message::Text(String::from("Hi")))
    .unwrap();

// Blocking
match proxy_output.channel.recv().unwrap() {
    networking::proxy::ProxyMessage::Forward(_msg) => {
        // Direct message from the remote
    }
    networking::proxy::ProxyMessage::ConnectionResetError => {
        // The proxy's connection has stopped, if auto_reconnect is set, the proxy will try to reconnect
        // If not, the proxy will exit
    }
    networking::proxy::ProxyMessage::Exit => {
        // The proxy encountered an error and exited
    }
}

// Non-blocking
let _server_msg_res = proxy_output.channel.try_recv();
```
