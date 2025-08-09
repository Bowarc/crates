#[test]
fn display() {
    use std::time::Duration;

    let one_nano = Duration::from_secs_f64(0.000000001);
    let one_micro = Duration::from_secs_f64(0.000001);
    let one_milli = Duration::from_secs_f64(0.001);
    let one_sec = Duration::from_secs(1);
    let one_minute = Duration::from_secs(60);
    let one_hour = Duration::from_secs(3600);
    let one_day = Duration::from_secs(86400);

    println!("one_nano: {}", time::format(&one_nano, -1));
    // one_nano: 1ns
    println!("one_micro: {}", time::format(&one_micro, -1));
    // one_micro: 1µs
    println!("one_milli: {}", time::format(&one_milli, -1));
    // one_milli: 1ms
    println!("one_sec: {}", time::format(&one_sec, -1));
    // one_sec: 1s
    println!("one_minute: {}", time::format(&one_minute, -1));
    // one_minute: 1m
    println!("one_hour: {}", time::format(&one_hour, -1));
    // one_hour: 1h
    println!("one_day: {}", time::format(&one_day, -1));
    // one_day: 1d
    println!(
        "Everything above: {}",
        time::format(
            &(one_nano + one_micro + one_milli + one_sec + one_minute + one_hour + one_day),
            -1
        )
    );
    // Everything above: 1d 1h 1m 1s 1ms 1µs 1ns

    println!();

    println!("{}", time::format(&Duration::from_secs(3661), -1));
    println!("{}", time::format(&Duration::from_secs(31546321), -1));
}
