use crate::fmap::{fmap, FMap};
use std::hash::Hash;

pub trait CollectVecCapacity<T> {
    fn collect_vec_cap(self, cap: usize) -> Vec<T>;
}

impl<T, I> CollectVecCapacity<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_vec_cap(self, cap: usize) -> Vec<T> {
        let mut v = Vec::with_capacity(cap.max(self.size_hint().0));
        v.extend(self);
        v
    }
}

pub trait CollectMapCapacity<K, V>: Iterator<Item = (K, V)> {
    fn collect_fmap_cap(self, cap: usize) -> FMap<K, V>;
}

impl<K, V, I> CollectMapCapacity<K, V> for I
where
    I: Iterator<Item = (K, V)>,
    K: Eq + Hash,
{
    fn collect_fmap_cap(self, cap: usize) -> FMap<K, V> {
        let mut v = fmap(cap.max(self.size_hint().0));
        v.extend(self);
        v
    }
}
