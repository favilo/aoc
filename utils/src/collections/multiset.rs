use std::hash::{BuildHasher, Hash};

use allocator_api2::alloc::{Allocator, Global};
use hashbrown::{DefaultHashBuilder, HashMap};

#[derive(Default, Clone)]
pub struct HashMultiSet<T, S = DefaultHashBuilder, A = Global>
where
    A: Allocator,
{
    inner: HashMap<T, usize, S, A>,
}

impl<T, S, A> PartialEq for HashMultiSet<T, S, A>
where
    A: Allocator,
    S: BuildHasher,
    T: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T, S, A> Eq for HashMultiSet<T, S, A>
where
    A: Allocator,
    S: BuildHasher,
    T: Hash + Eq,
{
}

impl<T, S, A> std::fmt::Debug for HashMultiSet<T, S, A>
where
    T: std::fmt::Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.inner.iter()).finish()
    }
}

impl<T, S, A> HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    S: Default + BuildHasher,
    A: Default + Allocator,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::with_hasher_in(S::default(), A::default()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher_in(capacity, S::default(), A::default()),
        }
    }

    pub fn insert(&mut self, v: T) {
        *self.inner.entry(v).or_default() += 1;
    }

    pub fn insert_many(&mut self, v: T, c: usize) {
        *self.inner.entry(v).or_default() += c;
    }

    pub fn contains(&self, v: &T) -> bool {
        self.inner.contains_key(v)
    }

    pub fn len(&self) -> usize {
        self.inner.iter().map(|(_, v)| v).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T, S, A> IntoIterator for HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    A: Allocator,
{
    type Item = (T, usize);

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T, S, A> FromIterator<(T, usize)> for HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
    A: Allocator + Default,
{
    fn from_iter<Inner: IntoIterator<Item = (T, usize)>>(iter: Inner) -> Self {
        let mut this = Self::new();
        iter.into_iter().for_each(|(v, c)| this.insert_many(v, c));
        this
    }
}

impl<T, S, A> FromIterator<T> for HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
    A: Allocator + Default,
{
    fn from_iter<Inner: IntoIterator<Item = T>>(iter: Inner) -> Self {
        let mut this = Self::new();
        iter.into_iter().for_each(|v| this.insert(v));
        this
    }
}

impl<T, S, A> Extend<(T, usize)> for HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
    A: Allocator + Default,
{
    fn extend<Inner: IntoIterator<Item = (T, usize)>>(&mut self, iter: Inner) {
        iter.into_iter().for_each(|(v, c)| self.insert_many(v, c));
    }
}

impl<T, S, A> Extend<T> for HashMultiSet<T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
    A: Allocator + Default,
{
    fn extend<Inner: IntoIterator<Item = T>>(&mut self, iter: Inner) {
        iter.into_iter().for_each(|v| self.insert(v));
    }
}
