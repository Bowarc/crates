use rand::{Rng,seq::SliceRandom};

#[cfg(any(feature = "bagU8", feature = "bagU16", feature = "bagU32", feature = "bagU64", feature = "bagU128"))]
mod weighted_bag;
#[cfg(any(feature = "bagU8", feature = "bagU16", feature = "bagU32", feature = "bagU64", feature = "bagU128"))]
pub use weighted_bag::WeightedBag;

/// Samples a number from a range (So nbr => min && nbr <max)
pub fn get<T>(x: T, y: T) -> T
where
    T: rand::distributions::uniform::SampleUniform
        + std::cmp::PartialEq + std::cmp::PartialOrd,
        // + std::fmt::Debug
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
        // + std::fmt::Debug
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
pub fn pick<T: std::fmt::Debug>(entry: &[T]) -> &T {
    if entry.is_empty() {
        panic!("Can't sample empty vec: {entry:?}")
    }
    entry.choose(&mut rand::thread_rng()).unwrap()
}
