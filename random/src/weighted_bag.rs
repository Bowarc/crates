mod entry;
mod weight;

use entry::WeightedBagEntry;
pub use weight::Weight;

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(from = "Vec<(T, W)>")
)]

/// A `WeightedBag` is a collection that holds entries of type `T` with associated weights of type `W`.  
/// The weights determine the likelihood of selecting each entry when retrieving a random item from the bag.
///
/// # Type Parameters
/// - `T`: The type of the entries stored in the bag.
/// - `W`: A type that implements the [Weight] trait, representing the weight of each entry.
///
/// # Features
/// This struct can derive `Serialize` and `Deserialize` traits when the `serde` feature is enabled.  
/// It can also be constructed from a vector of tuples `Vec<(T, W)>` containing entries and their corresponding weights.
///
/// # Example
/// ```
/// let mut bag: random::WeightedBag<&str, u32> = random::WeightedBag::default();
/// bag.add_entry("apple", 2);
/// bag.add_entry("banana", 1);
/// let random_fruit: Option<&&str> = bag.try_get_random();
/// ```

pub struct WeightedBag<T, W: Weight> {
    entries: Vec<WeightedBagEntry<T, W>>,
    weight: Option<W>,
}

impl<T, W: Weight> WeightedBag<T, W> {
    /// Adds an entry with given weight to the bag
    ///
    /// Panics if the weight is 0
    pub fn add_entry(&mut self, t: T, weight: W) {
        // Doesn't make sense + would break the system
        assert_ne!(weight, W::zero(), "Weightless entries are not allowed");

        // This is pretty ugly but the other way is to use a signed integer type, which would be dumb (waste of half the memory used)
        // We could use 0 as base and if 0 { self.w = w -1} but adding a weight of 1 as first entry would make the initialisation loop (+ sentinel values are stoopid to use when you have a rich type system)
        if let Some(acc_weight) = &mut self.weight {
            *acc_weight += weight;
        } else {
            self.weight = Some(weight - W::one());
        }

        self.entries.push(WeightedBagEntry {
            inner: t,
            weight: self.weight.clone().unwrap(),
        })
    }

    // I needed this part to be it's own method for tests, and since it's inlined, i don't see it being any different than not
    #[inline]
    pub(crate) fn get(&self, r: W) -> Option<&T> {
        self.entries.iter().find(|e| e.weight >= r).map(|e| &**e)
    }

    /// Retrieve a random entry from the bag, chances are based on weight
    pub fn try_get_random(&self) -> Option<&T> {
        let Some(acc_weight) = self.weight.clone() else {
            return None;
        };

        self.get(super::get_inc(W::zero(), acc_weight).into())
    }

    /// Short hand for [WeightedBag::try_get_random].unwrap()
    ///
    /// # Panics if:
    ///
    /// - The bag is empty
    #[inline]
    pub fn get_random(&self) -> &T {
        self.try_get_random().unwrap()
    }
}

impl<T, W: Weight> From<Vec<(T, W)>> for WeightedBag<T, W> {
    fn from(items: Vec<(T, W)>) -> Self {
        let mut new_bag = Self::default();
        items
            .into_iter()
            .for_each(|(item, weight)| new_bag.add_entry(item, weight));
        new_bag
    }
}

impl<T, W: Weight> Default for WeightedBag<T, W> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            weight: None,
        }
    }
}

impl<T: Clone, W: Weight> Clone for WeightedBag<T, W> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            weight: self.weight.clone(),
        }
    }
}

impl<T: std::fmt::Debug, W: Weight + std::fmt::Debug> std::fmt::Debug for WeightedBag<T, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WeightedBag")
            .field("entries", &self.entries)
            .field("total_weight", &self.weight)
            .finish()
    }
}

#[test]
fn test() {
    fn inner_test<T: num_traits::NumCast + Weight>() -> Option<()> {
        let bag = super::WeightedBag::<&str, T>::from(vec![
            ("Hi", T::from(2)?),         // 0..=1
            ("Hellow", T::from(1)?),     //  =2
            ("Bonjour", T::from(4)?),    //  3..=6
            ("Holà", T::from(4)?),       //  7..=10
            ("こんにちは", T::from(3)?), // 11..=13
            ("你好", T::from(10)?),      // 14..=23
            ("Olá", T::from(7)?),        // 24..=30
            ("Hej", T::from(5000)?),     // 31..=5030
        ]);

        // dbg!(&bag);

        assert_eq!(bag.get(T::zero()), Some(&"Hi"));
        assert_eq!(bag.get(T::one()), Some(&"Hi"));

        assert_eq!(bag.get(T::from(2)?), Some(&"Hellow"));

        assert_eq!(bag.get(T::from(3)?), Some(&"Bonjour"));
        assert_eq!(bag.get(T::from(6)?), Some(&"Bonjour"));

        assert_eq!(bag.get(T::from(7)?), Some(&"Holà"));
        assert_eq!(bag.get(T::from(10)?), Some(&"Holà"));

        assert_eq!(bag.get(T::from(11)?), Some(&"こんにちは"));
        assert_eq!(bag.get(T::from(13)?), Some(&"こんにちは"));

        assert_eq!(bag.get(T::from(14)?), Some(&"你好"));
        assert_eq!(bag.get(T::from(23)?), Some(&"你好"));

        assert_eq!(bag.get(T::from(24)?), Some(&"Olá"));
        assert_eq!(bag.get(T::from(30)?), Some(&"Olá"));

        assert_eq!(bag.get(T::from(31)?), Some(&"Hej"));
        assert_eq!(bag.get(T::from(5030)?), Some(&"Hej"));

        assert_eq!(bag.get(T::from(5031)?), None::<&&str>);

        Some(())
    }

    assert_eq!(inner_test::<u8>(), None::<()>); // Fails on T::from(5000)
    inner_test::<u16>().unwrap(); // should pass
    inner_test::<u32>().unwrap(); // should pass
    inner_test::<u64>().unwrap(); // should pass
    inner_test::<u128>().unwrap(); // should pass
}
