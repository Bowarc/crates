
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

    println!("{}", time::format(one_nano, -1));
    println!("{}", time::format(one_micro, -1));
    println!("{}", time::format(one_milli, -1));
    println!("{}", time::format(one_sec, -1));
    println!("{}", time::format(one_minute, -1));
    println!("{}", time::format(one_hour, -1));
    println!("{}", time::format(one_day, -1));


    println!("{}", time::format(Duration::from_secs(3661), -1));
    println!("{}", time::format(Duration::from_secs(31546321), -1));


}
