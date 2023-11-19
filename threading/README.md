## Bundle of std::sync::mpsc::Sender and std::sync::mpsc::Receiver with some added methods 

#### Use example:

cargo.toml
```toml
[dependencies]
threading = {git = "https://github.com/Bowarc/Crates.git", package = "threading"}
``` 
main.rs
```rust
#[derive(PartialEq)]
enum T1Msg {
    //
}
#[derive(PartialEq)] 
enum T2Msg {
    Hi,
    Hellow,
}

let (c1, c2) = threading::Channel::<T1Msg, T2Msg>::new_pair();

std::thread::spawn(move || c2.send(T2Msg::Hi));

let msg: T2Msg = c1.recv().unwrap();
// println!("{msg:?}"); // Hi

// Will return after 1.5 second or before if it received a msg
let _: Result<T2Msg, std::sync::mpsc::RecvTimeoutError> =
    c1.recv_timeout(std::time::Duration::from_secs_f32(1.5));

// will block and discard all message till it receive Hellow
// May return an error if c2 disconnected
let _: Result<(), std::sync::mpsc::RecvError> = c1.wait_for(T2Msg::Hellow);

// Will return after 2 seconds or when receiving Hellow
let _: Result<(), std::sync::mpsc::RecvTimeoutError> =
    c1.wait_for_or_timeout(T2Msg::Hellow, std::time::Duration::from_secs(2));
``` 