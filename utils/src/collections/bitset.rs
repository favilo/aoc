pub trait ToBitSetIndex {
    fn to_bitset_index(&self, dim: &Dim) -> usize;
}

impl<T> ToBitSetIndex for T
where
    T: Copy,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    fn to_bitset_index(&self, _dim: &Dim) -> usize {
        usize::try_from(*self).unwrap()
    }
}

pub trait FromBitSetIndex {
    fn from_bitset_index(index: usize, dim: &Dim) -> Self;
}

impl<T> FromBitSetIndex for T
where
    T: Copy,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn from_bitset_index(index: usize, _dim: &Dim) -> Self {
        Self::try_from(index).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dim {
    #[default]
    Unbounded,
    Fixed(Box<[usize]>),
}

pub trait Dimension {
    fn bounds(&self) -> Option<&[usize]>;
    fn capacity(&self) -> Option<usize>;
}

impl Dimension for Dim {
    fn bounds(&self) -> Option<&[usize]> {
        match self {
            Dim::Unbounded => None,
            Dim::Fixed(v) => Some(&v),
        }
    }

    fn capacity(&self) -> Option<usize> {
        match self {
            Dim::Unbounded => None,
            Dim::Fixed(v) => Some(v.iter().product()),
        }
    }
}

impl Dimension for [usize] {
    fn bounds(&self) -> Option<&[usize]> {
        Some(self)
    }

    fn capacity(&self) -> Option<usize> {
        Some(self.iter().product())
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Hash)]
pub struct Unbounded;

impl Dimension for Unbounded {
    fn bounds(&self) -> Option<&[usize]> {
        None
    }

    fn capacity(&self) -> Option<usize> {
        None
    }
}

impl From<usize> for Dim {
    fn from(value: usize) -> Self {
        Dim::Fixed(Box::new([value]))
    }
}

impl<const N: usize> From<[usize; N]> for Dim {
    fn from(value: [usize; N]) -> Self {
        Dim::Fixed(Box::new(value))
    }
}

impl<const N: usize> From<&[usize; N]> for Dim {
    fn from(value: &[usize; N]) -> Self {
        Dim::Fixed(Box::from(&value[..]))
    }
}

impl<'a> From<&'a [usize]> for Dim {
    fn from(value: &'a [usize]) -> Self {
        Dim::Fixed(Box::from(value))
    }
}

impl From<Unbounded> for Dim {
    fn from(_: Unbounded) -> Self {
        Dim::Unbounded
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct BitSet<T> {
    set: bit_set::BitSet,
    dim: Dim,
    _marker: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for BitSet<T>
where
    T: FromBitSetIndex + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitSet")
            .field_with("values", |f| {
                f.debug_set()
                    .entries(self.set.iter().map(|i| T::from_bitset_index(i, &self.dim)))
                    .finish()
            })
            .field("dim", &self.dim)
            .finish()
    }
}

impl<T> BitSet<T>
where
    T: ToBitSetIndex,
{
    pub fn unbounded() -> Self {
        Self {
            set: bit_set::BitSet::new(),
            dim: Dim::Unbounded,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_bounds<I>(dim: I) -> Self
    where
        Dim: From<I>,
    {
        let dim = Dim::from(dim);
        BitSet {
            set: bit_set::BitSet::with_capacity(dim.capacity().expect("invalid bounds")),
            dim,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value.to_bitset_index(&self.dim));
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value.to_bitset_index(&self.dim))
    }

    pub fn clear(&mut self) {
        self.set.clear();
    }
}

impl<T> BitSet<T>
where
    T: ToBitSetIndex + FromBitSetIndex,
{
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.into_iter()
    }
}

impl<T> Extend<T> for BitSet<T>
where
    T: ToBitSetIndex,
{
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        iter.into_iter().for_each(|i| self.insert(i));
    }
}

impl<'a, T> IntoIterator for &'a BitSet<T>
where
    T: FromBitSetIndex,
{
    type Item = T;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set
            .iter()
            .map(|index| T::from_bitset_index(index, &self.dim))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend() {
        let mut set = BitSet::with_bounds(&[100]);
        set.extend(0..5);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(!set.contains(&5));
    }
}
