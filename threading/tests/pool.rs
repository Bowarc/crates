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
#[should_panic(expected = "Expected panic")]
fn panic() {
    use threading::ThreadPool;

    let pool = ThreadPool::new(1);

    let panicking_future = pool.run(|| panic!("Hi"));

    panicking_future.wait()
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
