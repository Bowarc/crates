#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct WeightedBagEntry<T, W> {
    pub(crate) inner: T,
    pub(crate) weight: W,
}

impl<T: Clone, W: Clone> Clone for WeightedBagEntry<T, W> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            weight: self.weight.clone(),
        }
    }
}

impl<T, W> std::ops::Deref for WeightedBagEntry<T, W> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: std::fmt::Debug, W: std::fmt::Debug> std::fmt::Debug for WeightedBagEntry<T, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bag")
            .field("inner", &self.inner)
            .field("weight", &self.weight)
            .finish()
    }
}
