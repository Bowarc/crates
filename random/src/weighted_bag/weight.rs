/// A trait representing a type that can be used as a weight in a `WeightedBag`.
///
/// # Implemented for
/// - [u8]
/// - [u16]
/// - [u32]
/// - [u64]
/// - [u128]
pub trait Weight:
    rand::distributions::uniform::SampleUniform
    + num_traits::Unsigned
    + num_traits::Zero
    + num_traits::One
    + std::fmt::Debug
    + std::ops::AddAssign
    + Clone
    + PartialOrd
{
}

impl<
        T: rand::distributions::uniform::SampleUniform
            + num_traits::Unsigned
            + num_traits::Zero
            + num_traits::One
            + std::fmt::Debug
            + std::ops::AddAssign
            + Clone
            + PartialOrd,
    > Weight for T
{
}
