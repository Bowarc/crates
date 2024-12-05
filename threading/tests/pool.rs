#[test]
fn thread_pool() {
    use threading::pool::FutureState;
    use threading::ThreadPool;

    let pool = ThreadPool::new(1);

    let future_1 = pool.run(|| 5);

    let future_2 = pool.run(|| String::from("Hi"));

    let future_3 = pool.run(|| (String::from("Hi"), false));

    future_1.wait();
    assert_eq!(future_1.state(), FutureState::Done);
    assert_eq!(future_1.output(), 5);

    future_2.wait();
    assert_eq!(future_2.state(), FutureState::Done);
    assert_eq!(future_2.output(), String::from("Hi"));

    future_3.wait();
    assert_eq!(future_3.state(), FutureState::Done);
    assert_eq!(future_3.output(), (String::from("Hi"), false));
}

#[test]
#[should_panic(expected = "Can't read the output of a task that has not completed successfully")]
fn panic() {
    use threading::ThreadPool;

    let pool = ThreadPool::new(1);

    let panicking_future = pool.run(|| panic!("Expected panic"));

    panicking_future.wait();

    // This is the call that panics
    panicking_future.output();
}

#[test]
fn not_clone() {
    use std::cell::RefCell;
    use threading::pool::FutureState;
    use threading::ThreadPool;
    #[derive(Debug, PartialEq)]
    struct NotClone {
        data: RefCell<i32>,
    }

    let pool = ThreadPool::new(1);

    let f = pool.run(move || NotClone {
        data: RefCell::new(1),
    });

    f.wait();
    assert_eq!(f.state(), FutureState::Done);
    assert_eq!(
        f.output(),
        NotClone {
            data: RefCell::new(1)
        }
    );
}

#[test]
fn doc() {
    use threading::pool::FutureState;
    use threading::ThreadPool;

    let pool_handle_1 = ThreadPool::new(15); // How many threads you want

    // ThreadPools can be cloned and even shared across thread
    // They are just handles
    // When you drop all the handles, the remote threads will close
    let pool_handle_2 = pool_handle_1.clone();
    let pool_handle_3 = pool_handle_1.clone();
    let _pool_handle_4 = pool_handle_3.clone();

    // This will send the closure to a thread to be ran as soon as possible, non-blocking for the main thread.
    // The future received from pool.run is used to get the output of the closure.
    let future_1 = pool_handle_1.run(|| String::from("Hi"));

    // You can use
    future_1.wait();
    // to block the current thread until the closure has completed

    // use .state() to retreive the current state of your closure
    // Refer to the documentation for more informations about each state
    let state = future_1.state();
    assert_eq!(state, FutureState::Done);

    // To get the output of the future, use
    let _output = future_1.output();
    // assert_eq!(output, String::from("Hi"));
    // The output method consumes the value, calling it multiple times will cause a panic
    // This because the Future struct **does not** bind the output value to be Copy nor Clone
    // let _ = future_1.output(); // panics

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
    loop {
        // You can also check if the closure has finished it's execution with
        if future_3.is_done() {
            // At this point the state can only be Done or Panicked
            assert_eq!(future_3.state(), FutureState::Done);
            let (hi, f) = future_3.output();
            assert_eq!(hi, String::from("Hi"));
            assert_eq!(f, false);
            break
        }

        // Do whatever
    }

    // About panicking closures
    // Panicking in a closure is fine, the remote thread will stay alive
    let panicking_future = pool_handle_1.run(|| panic!("Expected panic"));

    // This call will not block the main thread indefinetly
    panicking_future.wait();
    // At this point, we know the closure has finished it's execution,
    // but we don't know that it hsuccessfully ran or panicked.

    // We can check that using the state
    assert_eq!(panicking_future.state(), FutureState::Panicked);

    // Calling this will panic as there is no output to fetch
    // panicking_future.output();
}
