pub mod coord;

#[must_use]
pub fn mean(l: &[usize]) -> f64 {
    let sum = l.iter().sum::<usize>();
    (sum as f64) / (l.len() as f64)
}

#[must_use]
pub fn median(l: &[usize]) -> usize {
    let len = l.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (l[mid - 1] + l[mid]) / 2
    } else {
        l[mid]
    }
}

#[must_use]
pub fn concat_numbers(a: usize, b: usize) -> usize {
    a * 10usize.pow(digit_count(b)) + b
}

pub fn digit_count(b: usize) -> u32 {
    b.ilog10() + 1
}

pub fn digits(n: usize) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    (0..digit_count(n))
        .map(move |exp| n / 10usize.pow(exp) % 10)
        .map(u8::try_from)
        .map(Result::unwrap)
}
