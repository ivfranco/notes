use std::{
    iter::FromIterator,
    ops::{BitAnd, BitOr, Deref, Sub},
};

#[derive(Clone, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub struct SortedUVec<T> {
    inner: Vec<T>,
}

impl<T: Ord> SortedUVec<T> {
    pub fn new<I>(inner: I) -> Self
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

    pub fn remove(&mut self, value: &T) {
        self.inner.retain(|v| v != value)
    }

    pub fn is_subset(&self, rhs: &Self) -> bool {
        self.iter().all(|v| rhs.contains(v))
    }

    pub fn is_superset(&self, rhs: &Self) -> bool {
        rhs.is_subset(self)
    }
}

impl<T: Ord + Clone> SortedUVec<T> {
    fn difference(&self, rhs: &Self) -> Self {
        let mut inner = self.inner.clone();
        inner.retain(|v| !rhs.contains(v));
        inner.into()
    }

    fn union(&self, rhs: &Self) -> Self {
        let mut this = self.clone();
        this.extend(rhs.iter().cloned());
        this
    }

    fn intersection(&self, rhs: &Self) -> Self {
        let mut inner = self.inner.clone();
        inner.retain(|v| rhs.contains(v));
        inner.into()
    }
}

impl<'a, T: Ord + Clone> BitOr for &'a SortedUVec<T> {
    type Output = SortedUVec<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl<'a, T: Ord + Clone> BitAnd for &'a SortedUVec<T> {
    type Output = SortedUVec<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<'a, T: Ord + Clone> Sub for &'a SortedUVec<T> {
    type Output = SortedUVec<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl<T> Deref for SortedUVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> IntoIterator for &'a SortedUVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

impl<T: Ord> Extend<T> for SortedUVec<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter);
        self.normalize();
    }
}
