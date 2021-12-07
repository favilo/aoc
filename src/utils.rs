pub fn parse_int<T>(b: &[u8]) -> T
where
    T: From<usize>,
{
    From::from(b.iter().fold(0, |a, c| a * 10 + (c & 0x0f) as usize))
}
