use std::{
    collections::LinkedList,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::ConcurrentSet;

#[derive(Clone)]
pub struct HandOverHandLinkedList<K, V> {
    head: Link<K, V>,
}

impl<K, V> HandOverHandLinkedList<K, V> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { head: Link::null() }
    }
}

impl<K, V> HandOverHandLinkedList<K, V>
where
    K: PartialEq,
    V: Clone,
{
    fn get_node(&self, key: &K) -> MutexGuard<Option<*mut Node<K, V>>> {
        let mut link = self.head.lock();

        while let Some(node_ptr) = *link {
            // # Safety
            // all `*mut Node<K, V>` is created by [Box::into_raw] with proper data and layout. A
            // reference is de-allocated only in [HandOverHandLinkedList::remove] and only after the
            // reference is no longer accessible from any other thread. This safety guarantee
            // applies to all dereference of `*mut Node<K, V>` in this file.
            let node = unsafe { node_ptr.as_mut().expect("null pointer") };
            if &node.key == key {
                break;
            }

            // must acquire the lock of the next node before dropping the lock of the current node
            link = node.next.lock();
        }

        link
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.get_node(key).map(|node_ptr| {
            let node = unsafe { node_ptr.as_mut().expect("null pointer") };
            node.value.clone()
        })
    }

    /// Always insert at the head, old values with the same key is shadowed by the new value as
    /// [HandOverHandLinkedList::get] traverses from the head of the list. When removing a key, all
    /// values of the same key is removed at once, hence old values are apparently "replaced" by the
    /// new value of the same key.
    pub fn insert(&self, key: K, value: V) {
        let mut node = Node::new(key, value);
        // must lock the old head first, otherwise a now removed node may be assigned to the next
        // field of the new head
        let mut lock = self.head.lock();
        node.next = Link::new(*lock);
        let node_ptr = Box::into_raw(Box::new(node));
        *lock = Some(node_ptr);
    }

    pub fn remove(&self, key: &K) {
        // links are named after the node they point to
        // prev_link: link points to prev_node
        // curr_link: link points to curr_node
        let mut prev_link = self.head.lock();

        while let Some(prev_node_ptr) = *prev_link {
            // `prev_link` is locked for the entire while block

            let prev_node = unsafe { prev_node_ptr.as_mut().expect("null pointer") };
            if &prev_node.key == key {
                // remove the head, must be treated differently as this [HandOverHandLinkedList]
                // doesn't start with a sentinel node.

                // only happens at the beginning of the traverse, it must have prev_link == self.head
                let curr_link = prev_node.next.lock();
                *prev_link = *curr_link;
                drop(curr_link);

                // # Safety
                // all `*mut Node<K, V>` is created by [Box::into_raw] with proper data and layout.
                // The deallocation is performed while holding a mutex to the head, the head is
                // removed from the list and no longer accessible on exit from this block.  The same
                // safety guarantees apply to all drops of `*mut Node<K, V>` in this file.
                unsafe {
                    let node = Box::from_raw(prev_node_ptr);
                    drop(node);
                }
                continue;
            }

            let mut curr_link = prev_node.next.lock();
            // curr_link is locked till the end of the while block

            if let Some(curr_node_ptr) = *curr_link {
                let curr_node = unsafe { curr_node_ptr.as_mut().expect("null pointer") };
                if &curr_node.key == key {
                    let next_link = curr_node.next.lock();
                    *curr_link = *next_link;
                    drop(next_link);

                    unsafe {
                        let node = Box::from_raw(curr_node_ptr);
                        drop(node);
                    }
                }
            }

            prev_link = curr_link;
        }
    }
}

impl<K, V> ConcurrentSet<K, V> for HandOverHandLinkedList<K, V>
where
    K: PartialEq,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        self.get(key)
    }

    fn insert(&self, key: K, value: V) {
        self.insert(key, value);
    }

    fn remove(&self, key: &K) {
        self.remove(key);
    }
}

struct Link<K, V> {
    /// The ownership rules of safe Rust won't allow concurrent mutable accesses to the link.
    next: Arc<Mutex<Option<*mut Node<K, V>>>>,
}

/// # Safety
/// Hand-over-hand linked list is a well-established concurrent data structure, as long as my
/// implementation in Rust is correct (a BIG assumption) the links will not be accessed in an unsafe
/// manner.
unsafe impl<K, V> Send for Link<K, V> {}
unsafe impl<K, V> Sync for Link<K, V> {}

impl<K, V> Link<K, V> {
    fn null() -> Self {
        Self::new(None)
    }

    fn new(link: Option<*mut Node<K, V>>) -> Self {
        Self {
            next: Arc::new(Mutex::new(link)),
        }
    }

    fn lock(&self) -> MutexGuard<Option<*mut Node<K, V>>> {
        // panic should be propagated to the top level of every thread
        self.next.lock().unwrap()
    }
}

impl<K, V> Clone for Link<K, V> {
    fn clone(&self) -> Self {
        Self {
            next: Arc::clone(&self.next),
        }
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    next: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            next: Link::null(),
        }
    }
}

#[derive(Clone)]
pub struct LockedLinkedList<K, V> {
    list: Arc<Mutex<LinkedList<(K, V)>>>,
}

impl<K, V> LockedLinkedList<K, V> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            list: Arc::new(Mutex::new(LinkedList::new())),
        }
    }
}

impl<K, V> ConcurrentSet<K, V> for LockedLinkedList<K, V>
where
    K: PartialEq,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let list = self.list.lock().unwrap();
        list.iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
            .cloned()
    }

    fn insert(&self, key: K, value: V) {
        let mut list = self.list.lock().unwrap();
        list.push_front((key, value));
    }

    fn remove(&self, key: &K) {
        let mut list = self.list.lock().unwrap();
        let mut cursor = list.cursor_front_mut();

        while let Some((k, _v)) = cursor.current() {
            if k == key {
                cursor.remove_current();
            } else {
                cursor.move_next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn static_rules() {
        fn traits<T: Send + Sync>() {}
        traits::<HandOverHandLinkedList<i32, i32>>();
    }

    #[test]
    fn concurrent_insert() {
        let list = HandOverHandLinkedList::new();
        let handles = (0..10)
            .map(|i| {
                let local = list.clone();
                thread::spawn(move || {
                    local.insert(i, i);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..10 {
            assert_eq!(list.get(&i), Some(i));
        }
    }

    #[test]
    fn concurrent_replace() {
        let list = HandOverHandLinkedList::new();
        let handles = (0..10)
            .map(|i| {
                let local = list.clone();
                thread::spawn(move || {
                    local.insert(i, i);
                    local.insert(i, i + 10);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..10 {
            assert_eq!(list.get(&i), Some(i + 10));
        }
    }

    #[test]
    fn remove_all() {
        let list = HandOverHandLinkedList::new();
        for i in 0..10 {
            list.insert(0, i);
        }

        list.remove(&0);
        assert_eq!(list.get(&0), None);
    }

    #[test]
    fn concurrent_remove() {
        let list = HandOverHandLinkedList::new();
        for i in 0..10 {
            list.insert(i, i);
        }

        let handles = (0..10)
            .map(|i| {
                let local = list.clone();
                thread::spawn(move || {
                    local.remove(&i);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..10 {
            assert_eq!(list.get(&i), None);
        }
    }
}
