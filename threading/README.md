## Threadpool and channels


#### Use example:

cargo.toml
```toml
[dependencies]
threading = {git = "https://github.com/Bowarc/Crates.git", package = "threading"}
```

main.rs
```rust
// ThreadPool s can be cloned and even shared across thread
// They are just handles 
// When you drop all the handles, the remote threads will close
let pool = ThreadPool::new(7);

let x = 0;
// This will send the closure to a thread to be ran as soon as possible, non-blocking for the main thread.
let future = pool.run(move || {
    std::thread::sleep(std::time::Duration::from_secs(3));
    x + 1
});

// The future received from pool.run is used to get the output of the closure.

// use 
future.wait();
// To block the current thread until the closure has returned

// You can also check if the closure has already returned with
future.is_done();
// Which will return false if it's not yet picked up by a thread or still running

// To get the output of the future, use
future.output();
// Which gives you a result, where it's guaranteed to be Ok() if the closure has finished
// Quick note, i havn't done anything for closure panics yet.



```

 
#### Bundle of std::sync::mpsc::Sender and std::sync::mpsc::Receiver with some added methods 

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

// Every channel can send to and receive from it's linked counterpart 
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
