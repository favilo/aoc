use std::ops::{Range, RangeInclusive, Sub};

pub trait RangeIncExt {
    fn inside(&self, other: &Self) -> bool;
    fn inside_or_surrounding(&self, other: &Self) -> bool {
        self.inside(other) || other.inside(self)
    }

    fn overlaps(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn intersect(&self, other: &Self) -> Self;
    fn size(&self) -> usize;
}

impl<T> RangeIncExt for RangeInclusive<T>
where
    T: Ord + Copy + Sub<T>,
    <T as Sub<T>>::Output: TryInto<usize>,
    <<T as Sub<T>>::Output as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn inside(&self, other: &Self) -> bool {
        other.contains(self.start()) && other.contains(self.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.contains(self.start())
            || other.contains(self.end())
            || self.contains(other.start())
            || self.contains(other.end())
    }

    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        self.overlaps(other)
            .then_some(*self.start().min(other.start())..=*self.end().max(other.end()))
    }

    fn intersect(&self, other: &Self) -> Self {
        *self.start().max(other.start())..=*self.end().min(other.end())
    }

    fn size(&self) -> usize {
        (*self.end() - *self.start()).try_into().unwrap()
    }
}

impl<T> RangeIncExt for Range<T>
where
    T: Ord + Copy + Sub<T>,
    <T as Sub<T>>::Output: TryInto<usize>,
    <<T as Sub<T>>::Output as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn inside(&self, other: &Self) -> bool {
        other.contains(&self.start) && other.contains(&self.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.contains(&self.start)
            || other.contains(&self.end)
            || self.contains(&other.start)
            || self.contains(&other.end)
    }

    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        self.overlaps(other)
            .then_some(self.start.min(other.start)..self.end.max(other.end))
    }

    fn intersect(&self, other: &Self) -> Self {
        self.start.max(other.start)..self.end.min(other.end)
    }

    fn size(&self) -> usize {
        (self.end - self.start).try_into().unwrap()
    }
}
