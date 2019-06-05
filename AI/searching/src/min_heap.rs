/// min heap with seperated key and value
/// a wrapper around std::collections::BinaryHeap
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> PartialEq for Pair<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, V> Eq for Pair<K, V> where K: Eq {}

impl<K, V> PartialOrd for Pair<K, V>
where
    K: Eq + PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&rhs.key).map(Ordering::reverse)
    }
}

impl<K, V> Ord for Pair<K, V>
where
    K: Eq + Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.key.cmp(&rhs.key).reverse()
    }
}

pub struct MinHeap<K, V> {
    heap: BinaryHeap<Pair<K, V>>,
}

impl<K, V> MinHeap<K, V>
where
    K: Eq + Ord,
{
    pub fn new() -> Self {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Pair { key, value })
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|pair| (pair.key, pair.value))
    }

    pub fn peek(&mut self) -> Option<(&K, &V)> {
        self.heap.peek().map(|pair| (&pair.key, &pair.value))
    }
}
