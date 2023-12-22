
#[test]
fn display() {
    use std::time::Duration;

    let one_nano = Duration::from_secs_f64(0.000000001001);
    let one_micro = Duration::from_secs_f64(0.000001001);
    let one_milli = Duration::from_secs_f64(0.001001);
    let one_sec = Duration::from_secs_f64(1.1);
    let one_minute = Duration::from_secs_f64(61.);
    let one_hour = Duration::from_secs(3661);
    let one_day = Duration::from_secs(90000);

    println!("{}", time::format(one_nano));
    println!("{}", time::format(one_micro));
    println!("{}", time::format(one_milli));
    println!("{}", time::format(one_sec));
    println!("{}", time::format(one_minute));
    println!("{}", time::format(one_hour));
    println!("{}", time::format(one_day));


    println!("{}", time::format(Duration::from_secs(3661)));
    println!("{}", time::format(Duration::from_secs(31536001)));
}
