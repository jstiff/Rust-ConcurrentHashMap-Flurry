use crossbeam::epoch::{Atomic, Gaurd, Shared};
use std::cell::UnsafeCell;

/// a node is each of the things that appear in a bin. Key, Value entry.
/// pub(crate) makes functions visible only within the current crate

pub(crate) enum BinEntry<K, V> {
    Node(Node<K, V>),
}

impl<K,V> BinEntry<K,V> where K: Eq, {
    pub(crate) fn find(&self, hash: u64, key: &K, guard: &'g Guard) -> Option<Shared<'g, Node<K,V>>>{
        match *self{
            BinEntry::Node(ref n) => {
                if n.hash == hash && &n.key == key {
                    return Some(n);
                }
                if n.next.load()
            }
        }
    }

}

/// Key-Value entry
/// Atomic is a "raw pointer" that is safe to share between threads. 
/// Any method that loads an Atomic pointer must be passed a reference to a 'Guard'.
pub(crate) struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<BinEntry<K, V>>,
    pub(crate) hash: u64,
}
