


#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct WeightedBagEntry<T> {
    pub(crate) inner: T,
    pub(crate) weight: super::Weight,
}

impl<T: Clone> Clone for WeightedBagEntry<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            weight: self.weight,
        }
    }
}

impl<T> std::ops::Deref for WeightedBagEntry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


impl<T: std::fmt::Debug> std::fmt::Debug for WeightedBagEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bag")
            .field("inner", &self.inner)
            .field("weight", &self.weight)
            .finish()
    }
}
