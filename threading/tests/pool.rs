#[test]
fn thread_pool() {
    use threading::pool::ThreadPool;

    let pool = ThreadPool::new(7);

    let h1 = pool.clone();
    let h2 = pool.clone();
    let h3 = pool.clone();

    let mut futures = (0..5)
        .map(|_| {
            let x = 0;
            h1.run(move || {
                std::thread::sleep(std::time::Duration::from_secs(3));
                x + 1
            })
        })
        .collect::<Vec<_>>();

    futures.append(
        &mut (0..5)
            .map(|_| {
                let x = 1;
                h2.run(move || {
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
                    x + 1
                })
            })
            .collect::<Vec<_>>(),
    );

    futures.append(
        &mut (0..5)
            .map(|_| {
                let x = 2;
                h3.run(move || {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    x + 1
                })
            })
            .collect::<Vec<_>>(),
    );
    for future in futures.into_iter() {
        future.wait().unwrap();
        let y = future.output().unwrap();
        println!("{y}");
    }
}
