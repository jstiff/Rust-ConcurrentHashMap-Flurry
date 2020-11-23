use crossbeam::epoch::Atomic;
use std::cell::UnsafeCell;

/// a node is each of the things that appear in a bin. Key, Value entry.
/// pub(crate) makes functions visible only within the current crate

pub(crate) enum BinEntry<K, V> {
    Node(Node<K, V>),
}

impl<K,V> BinEntry<K,V> where K: Eq, {
    pub(crate) fn find(&self, hash: u64, key: &K) -> Option<&Node<K,V>>{
        match *self{
            BinEntry::Node(ref start) =>{
                let mut n = Some(start);
                while let Some(n) = n;
            }
        }
    }

}

/// Key-Value entry
pub(crate) struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<BinEntry<K, V>>,
    pub(crate) hash: u64,
}
