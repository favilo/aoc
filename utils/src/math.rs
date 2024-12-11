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
    a * 10usize.pow(b.ilog10() + 1) + b
}

