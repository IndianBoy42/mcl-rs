use fxhash::FxHashMap;
use fxhash::FxHashSet;
pub type FSet<T> = FxHashSet<T>;
pub type FMap<K, V> = FxHashMap<K, V>;
// pub use fnv::FnvHasher;
// type FSet<T> = HashSet<T, BuildHasherDefault<FnvHasher>>;
// type FMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

pub fn fmap<K, V>(cap: usize) -> FMap<K, V> {
    FMap::with_capacity_and_hasher(cap, std::hash::BuildHasherDefault::default())
}

pub fn fset<V>(cap: usize) -> FSet<V> {
    FSet::with_capacity_and_hasher(cap, std::hash::BuildHasherDefault::default())
}