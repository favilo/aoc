#[derive(Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct HVec<T, const N: usize = 10>(heapless::Vec<T, N>);

impl<T, const N: usize> core::ops::Deref for HVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T, const N: usize> core::ops::DerefMut for HVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T, const N: usize> AsRef<[T]> for HVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, const N: usize> AsMut<[T]> for HVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
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
        heapless::Vec::from_iter(iter).into()
    }
}

impl<T, const N: usize> From<heapless::Vec<T, N>> for HVec<T, N> {
    fn from(v: heapless::Vec<T, N>) -> Self {
        Self(v)
    }
}
