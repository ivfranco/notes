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

pub fn filter_indices<'a, I: 'a, T, F: 'a>(iter: I, p: F) -> impl Iterator<Item = usize> + 'a
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> bool,
{
    iter.into_iter()
        .enumerate()
        .filter_map(move |(i, t)| if p(t) { Some(i) } else { None })
}
