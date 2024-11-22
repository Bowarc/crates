#[test]
fn thread_pool() {
    use threading::pool::FutureState;
    use threading::ThreadPool;

    let pool = ThreadPool::new(1);

    let mut future_1 = pool.run(|| 5);

    let mut future_2 = pool.run(|| String::from("Hi"));

    let mut future_3 = pool.run(|| (String::from("Hi"), false));

    let mut future_4 = pool.run(|| panic!("Hi"));

    future_1.wait();
    assert_eq!(future_1.state(), FutureState::Done);
    assert_eq!(future_1.output(), Some(5));

    future_2.wait();
    assert_eq!(future_2.state(), FutureState::Done);
    assert_eq!(future_2.output(), Some(String::from("Hi")));

    future_3.wait();
    assert_eq!(future_3.state(), FutureState::Done);
    assert_eq!(future_3.output(), Some((String::from("Hi"), false)));

    future_4.wait();
    assert_eq!(future_4.state(), FutureState::Panicked);
    assert_eq!(future_4.output(), None);
}
