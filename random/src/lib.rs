#[macro_use]
extern crate log;

#[cfg(feature = "bag")]
pub mod weighted_bag;

#[cfg(feature = "bag")]
pub use weighted_bag::WeightedBag;

struct Storage {
    seed: u64,
    rng: rand::rngs::SmallRng,
}

std::thread_local! {
    static STORAGE: std::cell::RefCell<Storage> = std::cell::RefCell::new({
    use {
        rand::{rngs::SmallRng, Rng, SeedableRng as _},
    };

    // This is ugly, but i need the seed
    let seed = SmallRng::from_entropy().gen::<u64>();

    trace!("Initializing with seed: {seed}");

    Storage {
        seed,
        // This is the fastest way to make multithreading i found
        rng: SmallRng::seed_from_u64(seed),
    }


    })
}

/// Sets the seed for the future queries
/// This is mostly usefull to make deterministic tests for games, or even bug hunts
pub fn set_seed(seed: u64) {
    use rand::{rngs::SmallRng, SeedableRng};
    STORAGE.with_borrow_mut(|storage| {
        storage.seed = seed;

        storage.rng = SmallRng::seed_from_u64(seed);
    });
}

/// Retrieves the seed
pub fn seed() -> u64 {
    STORAGE.with_borrow(|storage| storage.seed)
}

/// Samples a number from the range x..y
///
/// x has to be smaller than or equal to y
pub fn get<T>(x: T, y: T) -> T
where
    T: rand::distributions::uniform::SampleUniform
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    use rand::Rng as _;

    if x == y {
        // warn!("Can't sample empty range: {x:?} !");
        return x;
    };

    STORAGE.with_borrow_mut(|storage| storage.rng.gen_range(x..y))
}

/// Samples a number from the range x..=y
///
/// x has to be smaller than or equal to y
pub fn get_inc<T>(x: T, y: T) -> T
where
    // R: rand::distributions::uniform::SampleRange<T> + std::fmt::Debug,
    T: rand::distributions::uniform::SampleUniform
        + std::cmp::PartialEq
        + std::fmt::Debug
        + std::cmp::PartialOrd,
{
    use rand::Rng as _;

    if x == y {
        // warn!("Can't sample empty range: {x:?} !");
        return x;
    };

    STORAGE.with_borrow_mut(|storage| storage.rng.gen_range(x..=y))
}

/// Returns true 50% of the time
///
/// Technically not realistic as it cannot land on it's side :)
pub fn conflip() -> bool {
    use rand::Rng as _;

    STORAGE.with_borrow_mut(|storage| storage.rng.gen_bool(0.5))
}

/// Samples a random String with a given lengh
pub fn str(len: usize) -> String {
    use rand::distributions::{Alphanumeric, DistString};

    STORAGE.with_borrow_mut(|storage| Alphanumeric.sample_string(&mut storage.rng, len))
}

/// Select a random element from the given slice
///
/// Panics if the input slice is empty
pub fn pick<T: std::fmt::Debug>(input: &[T]) -> &T {
    use rand::seq::SliceRandom;

    if input.is_empty() {
        panic!("Can't sample empty slice ")
    }

    STORAGE.with_borrow_mut(|storage| input.choose(&mut storage.rng).unwrap())
}
