#[derive(Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct HVec<T, const N: usize = 10>(heapless::Vec<T, N>);

impl core::ops::Deref for HVec<usize> {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl core::ops::DerefMut for HVec<usize> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T, const N: usize> winnow::stream::Accumulate<T> for HVec<T, N>
where
    T: std::fmt::Debug,
{
    fn initial(_: Option<usize>) -> Self {
        Self(heapless::Vec::new())
    }

    fn accumulate(&mut self, acc: T) {
        self.0.push(acc).expect("heapless::Vec overflow");
    }
}

impl<T, const N: usize> FromIterator<T> for HVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(heapless::Vec::from_iter(iter))
    }
}
