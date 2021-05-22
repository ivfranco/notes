#![feature(linked_list_cursors)]

pub mod counter;
pub mod linked_list;

pub trait ConcurrentSet<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn insert(&self, key: K, value: V);
    fn remove(&self, key: &K);
}
