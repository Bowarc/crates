#[test]
fn thread_pool() {
    use threading::ThreadPool;

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

    while !futures.is_empty() {
        let mut index = 0;
        while index < futures.len() {
            let f = futures.get(index).unwrap();
            if f.is_done() {
                let output = futures.remove(index).output().unwrap();

                println!("Output: {output}");
                assert_eq!(pool.flying_tasks_count(), h1.flying_tasks_count());
                assert_eq!(pool.flying_tasks_count(), h2.flying_tasks_count());
                assert_eq!(pool.flying_tasks_count(), h3.flying_tasks_count());
                println!("Flying tasks count: {}", pool.flying_tasks_count(),);
            } else {
                index += 1;
            }
        }
    }
}
