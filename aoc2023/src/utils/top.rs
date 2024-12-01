use std::fmt::Debug;

use heapless::{
    binary_heap::{Max, Min},
    BinaryHeap,
};

#[derive(Default, Clone, Debug)]
pub struct TopK<T: Ord, const K: usize>(BinaryHeap<T, Max, K>);

impl<T, const K: usize> FromIterator<T> for TopK<T, K>
where
    T: Ord + Debug + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut this = BinaryHeap::<T, Min, K>::new();
        iter.into_iter().for_each(|i| {
            if this.len() == K {
                let smallest = this.pop().unwrap();
                this.push(i.max(smallest)).unwrap();
            } else {
                this.push(i).unwrap();
            }
        });
        let mut new = BinaryHeap::<_, Max, K>::new();
        (0..K).for_each(|_| {
            new.push(this.pop().unwrap_or_default()).unwrap();
        });
        Self(new)
    }
}

impl<'a, T, const K: usize> IntoIterator for &'a TopK<T, K>
where
    T: Ord,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
