use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::collections::bitset::{Dim, Dimension, FromBitSetIndex, ToBitSetIndex};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize);

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Coord {
    pub fn inside_limits<I>(&self, height: I, width: I) -> bool
    where
        I: TryInto<isize>,
        <I as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    {
        self.0 >= 0
            && self.0 < height.try_into().unwrap()
            && self.1 >= 0
            && self.1 < width.try_into().unwrap()
    }

    pub fn sq_magnitude(self) -> isize {
        self.0.pow(2) + self.1.pow(2)
    }

    pub fn manhattan_distance(self, other: Self) -> usize {
        (self.0 - other.0).unsigned_abs() + (self.1 - other.1).unsigned_abs()
    }

    pub fn abs(self) -> Coord {
        Self(self.0.abs(), self.1.abs())
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl From<Coord> for (isize, isize) {
    fn from(value: Coord) -> Self {
        (value.0, value.1)
    }
}

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> Self {
        (value.0 as usize, value.1 as usize)
    }
}

impl From<&Coord> for (usize, usize) {
    fn from(value: &Coord) -> Self {
        (value.0 as usize, value.1 as usize)
    }
}

impl ToBitSetIndex for Coord {
    fn to_bitset_index(&self, dim: &Dim) -> usize {
        let dim = dim.bounds().expect("invalid bounds");
        self.0 as usize + self.1 as usize * dim[0]
    }
}

impl FromBitSetIndex for Coord {
    fn from_bitset_index(index: usize, dim: &Dim) -> Self {
        let dim = dim.bounds().expect("invalid bounds");
        Self(
            (index % dim[0]).try_into().unwrap(),
            (index / dim[0]).try_into().unwrap(),
        )
    }
}
