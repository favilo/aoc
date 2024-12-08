use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub isize, pub isize);

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
}
