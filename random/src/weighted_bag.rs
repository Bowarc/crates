#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(from = "Vec<(T, i32)>")
)]
pub struct WeightedBag<T> {
    entries: Vec<WeightedBagEntry<T>>,
    weight: i32,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct WeightedBagEntry<T> {
    inner: T,
    weight: i32,
}

impl<T> WeightedBag<T> {
    /// Adds an entry with given weight to the bag
    pub fn add_entry(&mut self, t: T, weight: i32) {
        self.weight += weight;
        self.entries.push(WeightedBagEntry { inner: t, weight })
    }

    /// Retrieve a random entry from the bag, chances are based on weight
    pub fn try_get_random(&self) -> Option<&T> {
        let r = super::get_inc(0, self.weight);

        self.entries.iter().find(|e| e.weight > r).map(|e| &**e)
    }

    /// Unless you modified yourself the weight or the entries somehow, this should never panic
    pub fn get_random(&self) -> &T {
        self.try_get_random().unwrap()
    }
}

impl<T> Default for WeightedBag<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            weight: 0,
        }
    }
}

impl<T> From<Vec<(T, i32)>> for WeightedBag<T> {
    fn from(items: Vec<(T, i32)>) -> Self {
        let mut new_bag = Self::default();
        items
            .into_iter()
            .for_each(|(item, weight)| new_bag.add_entry(item, weight));
        new_bag
    }
}

impl<T: Clone> Clone for WeightedBag<T>{
    fn clone(&self) -> Self {
        Self{
            entries: self.entries.clone(),
            weight: self.weight
        }
    }
}

impl<T: Clone> Clone for WeightedBagEntry<T>{
    fn clone(&self) -> Self {
        Self{
            inner: self.inner.clone(),
            weight: self.weight
        }
    }
}

impl<T> std::ops::Deref for WeightedBagEntry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

