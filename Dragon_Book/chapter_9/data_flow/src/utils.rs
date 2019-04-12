use std::hash::Hash;

pub fn sorted<'a, I, T: 'a>(set: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    T: Ord + Hash + Clone,
{
    let mut sorted: Vec<_> = set.into_iter().collect();
    sorted.sort();
    sorted
}
