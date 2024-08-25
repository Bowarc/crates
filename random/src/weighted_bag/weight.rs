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
