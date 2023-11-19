use rand::seq::SliceRandom;
use rand::Rng;

/// Samples a number from a range (So nbr => min && nbr <max)
pub fn get<T>(x: T, y: T) -> T
where
    // R: rand::distributions::uniform::SampleRange<T> + std::fmt::Debug,
    T: rand::distributions::uniform::SampleUniform
        + std::cmp::PartialEq
        + std::fmt::Debug
        + std::cmp::PartialOrd,
{
    if x == y {
        // warn!("Can't sample empty range: {:?}", x..y);
        return x;
    };

    rand::thread_rng().gen_range(x..y)
}

/// Samples a number from a range (So nbr => min && nbr =<max)
pub fn get_inc<T>(x: T, y: T) -> T
where
    // R: rand::distributions::uniform::SampleRange<T> + std::fmt::Debug,
    T: rand::distributions::uniform::SampleUniform
        + std::cmp::PartialEq
        + std::fmt::Debug
        + std::cmp::PartialOrd,
{
    if x == y {
        // warn!("Can't sample empty range: {:?}", x..=y);
        return x;
    };

    rand::thread_rng().gen_range(x..=y)
}

pub fn conflip() -> bool {
    rand::thread_rng().gen_bool(0.5)
}

/// Samples a String with a given lengh
pub fn str(len: usize) -> String {
    use rand::distributions::Alphanumeric; // 0.8
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// only crashes when sampling from empty vec
pub fn pick<T: Clone + std::fmt::Debug>(entry: &[T]) -> T {
    if entry.is_empty() {
        panic!("Can't sample empty vec: {entry:?}")
    }
    entry.choose(&mut rand::thread_rng()).unwrap().clone()
}
