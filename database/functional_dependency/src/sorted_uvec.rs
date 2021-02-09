use std::{iter::FromIterator, ops::Deref};

#[derive(Clone)]
pub struct SortedUVec<T> {
    inner: Vec<T>,
}

impl<T: Ord> SortedUVec<T> {
    fn new<I>(inner: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let inner = inner.into_iter().collect();
        let mut this = Self { inner };
        this.normalize();
        this
    }

    fn normalize(&mut self) {
        self.inner.sort_unstable();
        self.inner.dedup();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|v| other.contains(v))
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
}

impl<T> Deref for SortedUVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: Ord> FromIterator<T> for SortedUVec<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::new(iter)
    }
}

impl<T: Ord> From<Vec<T>> for SortedUVec<T> {
    fn from(inner: Vec<T>) -> Self {
        let mut this = Self { inner };
        this.normalize();
        this
    }
}
