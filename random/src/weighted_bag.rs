mod entry;
use entry::WeightedBagEntry;

#[cfg(feature = "bagU8")]
type Weight = u8;
#[cfg(feature = "bagU16")]
type Weight = u16;
#[cfg(feature = "bagU32")]
type Weight = u32;
#[cfg(feature = "bagU64")]
type Weight = u64;
#[cfg(feature = "bagU128")]
type Weight = u128;


#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(from = "Vec<(T, i32)>")
)]
pub struct WeightedBag<T> {
    entries: Vec<WeightedBagEntry<T>>,
    weight: Option<Weight>,
}

impl<T> WeightedBag<T> {
    /// Adds an entry with given weight to the bag
    pub fn add_entry(&mut self, t: T, weight: Weight) {
        // Doesn't make sense + would break the system
        assert_ne!(weight, 0, "Weightless entries are not allowed");

        // This is pretty ugly but the other way is to use a signed integer type, which would be dumb (waste of half the memory used)
        // We could use 0 as base and if 0 { self.w = w -1} but adding a weight of 1 as first entry would make the initialisation loop (+ sentinel values are stoopid to use when you have a rich type system)
        if let Some(acc_weight) = &mut self.weight {
            *acc_weight += weight;
        } else {
            self.weight = Some(weight - 1);
        }

        self.entries.push(WeightedBagEntry {
            inner: t,
            weight: self.weight.unwrap(),
        })
    }

    // For tests
    #[inline]
    pub(crate) fn get(&self, r: Weight) -> Option<&T> {
        self.entries.iter().find(|e| e.weight >= r).map(|e| &**e)
    }

    /// Retrieve a random entry from the bag, chances are based on weight
    pub fn try_get_random(&self) -> Option<&T> {
        let Some(acc_weight) = self.weight else {
            return None;
        };

        self.get(super::get_inc(0, acc_weight).into())
    }

    /// Panics if:
    ///     - The bag is empty
    ///     - You modified the entries or weight yourself somehow
    pub fn get_random(&self) -> &T {
        self.try_get_random().unwrap()
    }
}

impl<T> From<Vec<(T, Weight)>> for WeightedBag<T> {
    fn from(items: Vec<(T, Weight)>) -> Self {
        let mut new_bag = Self::default();
        items
            .into_iter()
            .for_each(|(item, weight)| new_bag.add_entry(item, weight));
        new_bag
    }
}

impl<T> Default for WeightedBag<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            weight: None,
        }
    }
}

impl<T: Clone> Clone for WeightedBag<T> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            weight: self.weight.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for WeightedBag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bag")
            .field("entries", &self.entries)
            .field("total_weight", &self.weight)
            .finish()
    }
}

#[test]
fn test() {
    let mut bag = super::WeightedBag::<&str>::default();

    bag.add_entry("Hi", 2); // 0..=1
    bag.add_entry("Hellow", 1); //  =2
    bag.add_entry("Bonjour", 4); //  3..=6
    bag.add_entry("Holà", 4); //  7..=10
    bag.add_entry("こんにちは", 3); // 11..=13
    bag.add_entry("你好", 10); // 14..=23
    bag.add_entry("Olá", 7); // 24..=30
    bag.add_entry("Hej", 5000); // 31..=5030

    dbg!(&bag);

    assert_eq!(bag.get(0), Some(&"Hi"));
    assert_eq!(bag.get(1), Some(&"Hi"));

    assert_eq!(bag.get(2), Some(&"Hellow"));

    assert_eq!(bag.get(3), Some(&"Bonjour"));
    assert_eq!(bag.get(6), Some(&"Bonjour"));

    assert_eq!(bag.get(7), Some(&"Holà"));
    assert_eq!(bag.get(10), Some(&"Holà"));

    assert_eq!(bag.get(11), Some(&"こんにちは"));
    assert_eq!(bag.get(13), Some(&"こんにちは"));

    assert_eq!(bag.get(14), Some(&"你好"));
    assert_eq!(bag.get(23), Some(&"你好"));

    assert_eq!(bag.get(24), Some(&"Olá"));
    assert_eq!(bag.get(30), Some(&"Olá"));

    assert_eq!(bag.get(31), Some(&"Hej"));
    assert_eq!(bag.get(5030), Some(&"Hej"));

    assert_eq!(bag.get(5031), None::<&&str>);
}
