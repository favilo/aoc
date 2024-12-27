use num::{traits::NumOps, One, Signed, Unsigned, Zero};

#[must_use]
pub fn parse_uint<N>(b: impl AsRef<[u8]>) -> N
where
    N: NumOps<usize, N> + NumOps<N> + Zero + One + From<u8> + Unsigned,
{
    b.as_ref()
        .iter()
        .fold(N::zero(), |a, c| a * 10 + N::from(c & 0x0f))
}

#[must_use]
pub fn parse_int<N>(b: impl AsRef<[u8]>) -> N
where
    N: NumOps<isize, N> + NumOps<N> + Zero + One + From<u8> + Signed,
{
    let mut b = b.as_ref();
    let multiplier = if b.first() == Some(&b'-') {
        b = &b[1..];
        -N::one()
    } else {
        N::one()
    };
    multiplier * b.iter().fold(N::zero(), |a, c| a * 10 + N::from(c & 0x0f))
}

#[must_use]
pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}
