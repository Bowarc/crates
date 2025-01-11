use std::{assert_eq, assert_ne};

#[test]
fn all() {
    // Delay of 1.5 second
    let mut delay = time::DTDelay::new(1.5);
    // Update it with delta time
    delay.update(0.2);
    delay.update(1.3);
    if delay.ended() {
        println!("Woooo");
        delay.restart();
    }

    delay.update(2.);
    // Use this over `.restart` when you have low framerate or want extreme precision
    let overtime = delay.time_since_ended(); // 0.5
                                             // Restart the timer but fake 0.5 seconds of elapsed time
    delay.restart_custom_timeline(overtime);

    let mut stopwatch = time::Stopwatch::start_new();

    // You can read it and let it run
    let one: std::time::Duration = stopwatch.read();

    let two: std::time::Duration = stopwatch.read();

    assert_ne!(one, two);

    // Or stop it to read it later
    stopwatch.stop();
    let one: std::time::Duration = stopwatch.read();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.25));
    let two: std::time::Duration = stopwatch.read();

    assert_eq!(one, two);

    // Formatting
    let d = std::time::Duration::from_secs(3600);
    println!("{}", time::format(d, -1)); // 1h

    // Time a function
    let fn1 = |x: i32| -> bool {
        // Heavy computation
        if x > 1 {
            return true;
        }
        false
    };

    let (fn_out, dur): (bool, std::time::Duration) = time::timeit(|| fn1(15));

    println!(
        "fn1 ran for {} and returnred {}",
        time::format(dur, 1),
        fn_out
    );
    // fn1 ran for 200ns and returnred true

    let fn2 = || -> i32 {
        std::thread::sleep(std::time::Duration::from_secs_f32(1.2));
        15
    };

    let (fn_out, dur): (i32, std::time::Duration) = time::timeit(fn2);

    println!(
        "fn2 ran for {} and returnred {}",
        time::format(dur, 2),
        fn_out
    );
    // fn2 ran for 1s 200ms and returnred 15

    // Mutable args
    let mut x = 20;

    let fn3 = |x: &mut i32| {
        *x -= 5;
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5))
    };

    let (fn_out, dur): ((), std::time::Duration) = time::timeit_mut(|| fn3(&mut x));

    println!(
        "fn3 ran for {} and returnred {:?}",
        time::format(dur, 1),
        fn_out
    );
    // fn3 ran for 500ms and returnred ()
}
