## Channels, Threadpools and sync futures.

### Documentation

The documentation for this crate can be found [here](https://bowarc.github.io/crates/threading)

#### Use example:

Cargo.toml
```toml
[dependencies]
threading = {git = "https://github.com/Bowarc/Crates.git", package = "threading"}
```

```rust
use threading::pool::FutureState;
use threading::ThreadPool;

let pool_handle_1 = ThreadPool::new(15); // How many threads you want

// ThreadPools can be cloned and even shared across thread
// They are just handles
// When you drop all the handles, the remote threads will close
let pool_handle_2 = pool_handle_1.clone();
let _pool_handle_3 = pool_handle_1.clone();
let _pool_handle_4 = _pool_handle_3.clone();

// This will send the closure to a thread to be ran as soon as possible, non-blocking for the main thread.
// The future received from pool.run is used to get the output of the closure.
let future_1 = pool_handle_1.run(|| {
    String::from("Hi")
});

// You can use 
future_1.wait(); 
// to block the current thread until the closure has completed

// use .state() to retreive the current state of your closure
// Refer to the documentation for more informations about each state
let state = future_1.state(); // assert_eq!(state, FutureState::Done);

// To get the output of the future, use
let output = future_1.output(); // assert_eq!(output, String::from("Hi"));
// The output method consumes the value, calling it multiple times will cause a panic
// This because the Future struct does not bind the output value to be Copy nor Clone
let _ = future_1.output(); // panics


let x = 4;
// You can use any handle, it doesn't matter, the same threads will exectute the closures
let future_2 = pool_handle_2.run(move || {
    std::thread::sleep(std::time::Duration::from_secs(3));
    x + 1
});
future_2.wait();
assert_eq!(future_2.state(), FutureState::Done);
assert_eq!(future_2.output(), 5);


let future_3 = pool_handle_1.run(|| (String::from("Hi"), false));
loop{
    // You can also check if the closure has finished it's execution with
    if future_3.is_done(){
        // At this point the state can only be Done or Panicked
        assert_eq!(future_3.state(), FutureState::Done);
        let (hi, f) = future_3.output();
        assert_eq!(hi, String::from("Hi"));
        assert_eq!(f, false);
    }

    // Do whatever
}


// About panicking closures
// Panicking in a closure is fine, the remote thread will stay alive
let panicking_future = pool_handle_1.run(|| panic!("Expected panic"));

// This call will not block the main thread indefinetly
panicking_future.wait();
// At this point, we know the closure has finished it's execution, 
// but we don't know if it has successfully ran or panicked.

// We can check that using the state
assert_eq!(panicking_future.state(), FutureState::Panicked);

// Calling this will panic as there is no output to fetch
panicking_future.output();
```

 
#### Bundle of std::sync::mpsc::Sender and std::sync::mpsc::Receiver with some added methods 

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
