use std::{
    borrow::Borrow,
    default::Default,
    fmt::Debug,
    hash::{BuildHasher, Hash},
    iter::once,
    ops::Index,
};

use allocator_api2::alloc::{Allocator, Global};

use hashbrown::{
    hash_map::{self, Entry, Keys},
    DefaultHashBuilder, HashMap, HashSet,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Clone)]
pub struct MultiMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    inner: HashMap<K, HashSet<V, S, A>, S, A>,
}

impl<K, V> Default for MultiMap<K, V, DefaultHashBuilder, Global> {
    fn default() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }
}

impl<K, V> MultiMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }
}

impl<K, V, S> MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: Default + Clone,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            inner: HashMap::with_hasher(hash_builder),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }
}

impl<K, V, S, A> MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    pub fn with_hasher_in(hash_builder: S, allocator: A) -> Self {
        Self {
            inner: HashMap::with_hasher_in(hash_builder, allocator),
        }
    }

    pub fn with_capacity_and_hasher_in(capacity: usize, hash_builder: S, allocator: A) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher_in(capacity, hash_builder, allocator),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        match self.inner.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(v);
            }
            Entry::Vacant(entry) => {
                entry.insert(HashSet::from_iter(once(v)));
            }
        }
    }

    pub fn insert_many(&mut self, k: K, v: impl IntoIterator<Item = V>) {
        match self.inner.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().extend(v);
            }
            Entry::Vacant(entry) => {
                entry.insert(HashSet::from_iter(v));
            }
        }
    }

    pub fn insert_many_from_slice<'a>(&mut self, k: K, v: impl IntoIterator<Item = &'a V>)
    where
        V: 'a + Clone,
    {
        self.inner
            .entry(k)
            .or_default()
            .extend(v.into_iter().cloned());
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.inner.contains_key(k)
    }

    pub fn len(&self) -> usize {
        self.inner.iter().map(|(_, v)| v.len()).sum()
    }

    pub fn remove(&mut self, k: &K) -> Option<HashSet<V, S, A>> {
        self.inner.remove(k)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k)?.iter().next()
    }

    pub fn get_all<Q>(&self, k: &Q) -> Option<&HashSet<V, S, A>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k)
    }

    pub fn get_all_mut<Q>(&mut self, k: &Q) -> Option<&mut HashSet<V, S, A>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_mut(k)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn keys(&self) -> Keys<'_, K, HashSet<V, S, A>> {
        self.inner.keys()
    }

    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, HashSet<V, S, A>> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, K, HashSet<V, S, A>> {
        self.inner.iter_mut()
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = (&'_ K, &'_ V)> {
        self.inner
            .iter()
            .flat_map(|(k, v)| v.iter().map(move |v| (k, v)))
    }

    pub fn entry(&mut self, k: K) -> hashbrown::hash_map::Entry<'_, K, HashSet<V, S, A>, S, A> {
        self.inner.entry(k)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.inner.iter_mut().for_each(|(k, set)| {
            set.retain(|v| f(k, v));
        });
        self.inner.retain(|_, set| !set.is_empty());
    }
}

impl<'a, K, V, S, A, Q> Index<&'a Q> for MultiMap<K, V, S, A>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Output = V;

    fn index(&self, index: &'a Q) -> &Self::Output {
        self.inner
            .get(index)
            .expect("no entry found for key")
            .iter()
            .next()
            .expect("no value found for key")
    }
}

impl<K, V, S, A> Debug for MultiMap<K, V, S, A>
where
    K: Eq + Hash + Debug,
    V: Eq + Hash + Debug,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S, A> PartialEq for MultiMap<K, V, S, A>
where
    K: Eq + Hash + PartialEq,
    V: Eq + Hash + PartialEq,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V, S, A> Eq for MultiMap<K, V, S, A>
where
    K: Eq + Hash + Eq,
    V: Eq + Hash + Eq,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
}

impl<K, V, S, A> FromIterator<(K, V)> for MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert(k, v));
        this
    }
}

impl<'a, K, V, S, A> FromIterator<(K, &'a [V])> for MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash + Clone,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'a [V])>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert_many_from_slice(k, v.as_ref()));
        this
    }
}

impl<K, V, S, A> FromIterator<(K, Vec<V>)> for MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash + Clone,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, Vec<V>)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert_many(k, v));
        this
    }
}

impl<'a, K, V, S, A> IntoIterator for &'a MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (&'a K, &'a HashSet<V, S, A>);
    type IntoIter = hash_map::Iter<'a, K, HashSet<V, S, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, A> IntoIterator for &'a mut MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (&'a K, &'a mut HashSet<V, S, A>);
    type IntoIter = hash_map::IterMut<'a, K, HashSet<V, S, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S, A> IntoIterator for MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (K, HashSet<V, S, A>);
    type IntoIter = hash_map::IntoIter<K, HashSet<V, S, A>, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V, S, A> IntoParallelIterator for &'a MultiMap<K, V, S, A>
where
    K: Eq + Hash + Sync + Send,
    V: Eq + Hash + Sync + Send,
    S: BuildHasher + Default + Clone + Sync + Send,
    A: Allocator + Default + Sync + Send,
{
    type Iter = impl ParallelIterator<Item = Self::Item>;

    type Item = (&'a K, &'a HashSet<V, S, A>);

    fn into_par_iter(self) -> Self::Iter {
        self.inner.par_iter()
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for MultiMap<K, V, S, A>
where
    K: Eq + Hash + Copy,
    V: Eq + Hash + Copy,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(&k, &v)| self.insert(k, v));
    }
}

impl<K, V, S, A, Inner> Extend<(K, Inner)> for MultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Inner: IntoIterator<Item = V>,
{
    fn extend<T: IntoIterator<Item = (K, Inner)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(k, v)| self.insert_many(k, v));
    }
}

#[derive(Default, Clone)]
pub struct OrderedMultiMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    inner: HashMap<K, Vec<V>, S, A>,
}

impl<K, V> OrderedMultiMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }
}

impl<K, V, S> OrderedMultiMap<K, V, S>
where
    K: Eq + Hash,
    S: Default + Clone,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            inner: HashMap::with_hasher(hash_builder),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }
}

impl<K, V, S, A> OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    pub fn with_hasher_in(hash_builder: S, allocator: A) -> Self {
        Self {
            inner: HashMap::with_hasher_in(hash_builder, allocator),
        }
    }

    pub fn with_capacity_and_hasher_in(capacity: usize, hash_builder: S, allocator: A) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher_in(capacity, hash_builder, allocator),
        }
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.inner.contains_key(k)
    }

    pub fn len(&self) -> usize {
        self.inner.iter().map(|(_, v)| v.len()).sum()
    }

    pub fn remove(&mut self, k: &K) -> Option<Vec<V>> {
        self.inner.remove(k)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k)?.iter().next()
    }

    pub fn get_all<Q>(&self, k: &Q) -> Option<&[V]>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k).map(|v| v.as_slice())
    }

    pub fn get_all_mut<Q>(&mut self, k: &Q) -> Option<&mut [V]>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_mut(k).map(|v| v.as_mut_slice())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn keys(&self) -> Keys<'_, K, Vec<V>> {
        self.inner.keys()
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.inner.entry(k).or_default().push(v);
    }

    pub fn insert_many(&mut self, k: K, v: impl IntoIterator<Item = V>) {
        self.inner.entry(k).or_default().extend(v);
    }

    pub fn insert_many_from_ref<'a>(&mut self, k: K, v: impl IntoIterator<Item = &'a V>)
    where
        V: 'a + Clone,
    {
        self.inner
            .entry(k)
            .or_default()
            .extend(v.into_iter().cloned());
    }

    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, Vec<V>> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, K, Vec<V>> {
        self.inner.iter_mut()
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = (&'_ K, &'_ V)> {
        self.inner
            .iter()
            .flat_map(|(k, v)| v.iter().map(move |v| (k, v)))
    }

    pub fn flat_iter_mut(&mut self) -> impl Iterator<Item = (&'_ K, &'_ mut V)> {
        self.inner
            .iter_mut()
            .flat_map(|(k, v)| v.iter_mut().map(move |v| (k, v)))
    }

    pub fn entry(&mut self, k: K) -> hashbrown::hash_map::Entry<'_, K, Vec<V>, S, A> {
        self.inner.entry(k)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.inner.iter_mut().for_each(|(k, set)| {
            set.retain(|v| f(k, v));
        });
        self.inner.retain(|_, set| !set.is_empty());
    }
}

impl<'a, K, V, S, A, Q> Index<&'a Q> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Output = V;

    fn index(&self, index: &'a Q) -> &Self::Output {
        self.inner
            .get(index)
            .expect("no entry found for key")
            .iter()
            .next()
            .expect("no value found for key")
    }
}

impl<K, V, S, A> Debug for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash + Debug,
    V: Eq + Hash + Debug,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S, A> PartialEq for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash + PartialEq,
    V: Eq + Hash + PartialEq,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V, S, A> Eq for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash + Eq,
    V: Eq + Hash + Eq,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
}

impl<K, V, S, A> FromIterator<(K, V)> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert(k, v));
        this
    }
}

impl<'a, K, V, S, A> FromIterator<(K, &'a [V])> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash + Clone,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'a [V])>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert_many_from_ref(k, v));
        this
    }
}

impl<K, V, S, A> FromIterator<(K, Vec<V>)> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash + Clone,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Self: Default,
{
    fn from_iter<T: IntoIterator<Item = (K, Vec<V>)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size_hint = iter.size_hint().0;

        let mut this =
            Self::with_capacity_and_hasher_in(size_hint, Default::default(), Default::default());
        iter.for_each(|(k, v)| this.insert_many(k, v));
        this
    }
}

impl<'a, K, V, S, A> IntoIterator for &'a OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (&'a K, &'a Vec<V>);
    type IntoIter = hash_map::Iter<'a, K, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, A> IntoIterator for &'a mut OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (&'a K, &'a mut Vec<V>);
    type IntoIter = hash_map::IterMut<'a, K, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S, A> IntoIterator for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    type Item = (K, Vec<V>);
    type IntoIter = hash_map::IntoIter<K, Vec<V>, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash + Copy,
    V: Eq + Hash + Copy,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(&k, &v)| self.insert(k, v));
    }
}

impl<K, V, S, A, Inner> Extend<(K, Inner)> for OrderedMultiMap<K, V, S, A>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default + Clone,
    A: Allocator + Default,
    Inner: IntoIterator<Item = V>,
{
    fn extend<T: IntoIterator<Item = (K, Inner)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(k, v)| self.insert_many(k, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _: MultiMap<usize, usize> = MultiMap {
            inner: HashMap::new(),
        };

        let _: OrderedMultiMap<usize, usize> = OrderedMultiMap {
            inner: HashMap::new(),
        };
    }

    #[test]
    fn new() {
        let _: MultiMap<usize, usize> = MultiMap::new();

        let _: OrderedMultiMap<usize, usize> = OrderedMultiMap::new();
    }

    #[test]
    fn with_capacity() {
        let _: MultiMap<usize, usize> = MultiMap::with_capacity(20);

        let _: OrderedMultiMap<usize, usize> = OrderedMultiMap::with_capacity(20);
    }

    #[test]
    fn insert() {
        let mut map = MultiMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get_all(&1), Some(&HashSet::from_iter([2])));
        assert_eq!(map.get_all(&3), Some(&HashSet::from_iter([4])));

        let mut map = OrderedMultiMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get_all(&1), Some(&[2][..]));
        assert_eq!(map.get_all(&3), Some(&[4][..]));
    }

    #[test]
    fn insert_identical() {
        let mut map = MultiMap::new();
        map.insert(1, 42);
        map.insert(1, 42);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get_all(&1), Some(&HashSet::from_iter([42])));

        let mut map = OrderedMultiMap::new();
        map.insert(1, 42);
        map.insert(1, 42);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get_all(&1), Some(&[42, 42][..]));
    }

    #[test]
    fn insert_many() {
        let mut map = MultiMap::new();
        map.insert_many(1, Some(2));
        map.insert_many(3, vec![4, 5]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get_all(&1), Some(&HashSet::from_iter([2])));
        assert_eq!(map.get_all(&3), Some(&HashSet::from_iter([4, 5])));

        let mut map = OrderedMultiMap::new();
        map.insert_many(1, Some(2));
        map.insert_many(3, vec![4, 5]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get_all(&1), Some(&[2][..]));
        assert_eq!(map.get_all(&3), Some(&[4, 5][..]));
    }

    #[test]
    fn insert_many_from_ref() {
        let mut map = MultiMap::new();
        map.insert_many_from_slice(1, Some(&2));
        map.insert_many_from_slice(3, &[4, 5]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get_all(&1), Some(&HashSet::from_iter([2])));
        assert_eq!(map.get_all(&3), Some(&HashSet::from_iter([4, 5])));

        let mut map = OrderedMultiMap::new();
        map.insert_many_from_ref(1, Some(&2));
        map.insert_many_from_ref(3, &[4, 5]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get_all(&1), Some(&[2][..]));
        assert_eq!(map.get_all(&3), Some(&[4, 5][..]));
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn index_no_entry_set() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        let _ = &m[&1];
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn index_no_entry_ordered() {
        let m: OrderedMultiMap<usize, usize> = OrderedMultiMap::new();
        let _ = &m[&1];
    }

    #[test]
    fn index() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 41);
        m.insert(2, 42);
        m.insert(3, 43);
        let values = m[&2];
        assert_eq!(values, 42);

        let mut m: OrderedMultiMap<usize, usize> = OrderedMultiMap::new();
        m.insert(1, 41);
        m.insert(2, 42);
        m.insert(3, 43);
        let values = m[&2];
        assert_eq!(values, 42);
    }

    #[test]
    fn remove_not_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        let v = m.remove(&1);
        assert_eq!(v, None);

        let mut m: OrderedMultiMap<usize, usize> = OrderedMultiMap::new();
        let v = m.remove(&1);
        assert_eq!(v, None);
    }

    #[test]
    fn remove_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        let v = m.remove(&1);
        assert_eq!(v, Some(HashSet::from_iter([42])));

        let mut m: OrderedMultiMap<usize, usize> = OrderedMultiMap::new();
        m.insert(1, 42);
        let v = m.remove(&1);
        assert_eq!(v, Some(vec![42]));
    }

    #[test]
    fn test_from_iterator() {
        let vals: Vec<(&str, i64)> = vec![("foo", 123), ("bar", 456), ("foo", 789)];
        let multimap: MultiMap<&str, i64> = MultiMap::from_iter(vals.clone());

        let foo_vals = multimap.get_all("foo").unwrap();
        assert!(foo_vals.contains(&123));
        assert!(foo_vals.contains(&789));

        let bar_vals = multimap.get_all("bar").unwrap();
        assert!(bar_vals.contains(&456));

        let multimap: OrderedMultiMap<&str, i64> = OrderedMultiMap::from_iter(vals);

        let foo_vals = multimap.get_all("foo").unwrap();
        assert!(foo_vals.contains(&123));
        assert!(foo_vals.contains(&789));

        let bar_vals = multimap.get_all("bar").unwrap();
        assert!(bar_vals.contains(&456));
    }

    #[test]
    fn test_from_vec_iterator() {
        let vals: Vec<(&str, Vec<i64>)> = vec![
            ("foo", vec![123, 456]),
            ("bar", vec![234]),
            ("foobar", vec![567, 678, 789]),
            ("bar", vec![12, 23, 34]),
        ];

        let multimap: MultiMap<&str, i64> = MultiMap::from_iter(vals.clone());

        let foo_vals = multimap.get_all("foo").unwrap();
        assert!(foo_vals.contains(&123));
        assert!(foo_vals.contains(&456));

        let bar_vals = multimap.get_all("bar").unwrap();
        assert!(bar_vals.contains(&234));
        assert!(bar_vals.contains(&12));
        assert!(bar_vals.contains(&23));
        assert!(bar_vals.contains(&34));

        let foobar_vals = multimap.get_all("foobar").unwrap();
        assert!(foobar_vals.contains(&567));
        assert!(foobar_vals.contains(&678));
        assert!(foobar_vals.contains(&789));

        let multimap: OrderedMultiMap<&str, i64> = OrderedMultiMap::from_iter(vals);

        let foo_vals = multimap.get_all("foo").unwrap();
        assert!(foo_vals.contains(&123));
        assert!(foo_vals.contains(&456));

        let bar_vals = multimap.get_all("bar").unwrap();
        assert!(bar_vals.contains(&234));
        assert!(bar_vals.contains(&12));
        assert!(bar_vals.contains(&23));
        assert!(bar_vals.contains(&34));

        let foobar_vals = multimap.get_all("foobar").unwrap();
        assert!(foobar_vals.contains(&567));
        assert!(foobar_vals.contains(&678));
        assert!(foobar_vals.contains(&789));
    }
}
