#[must_use]
pub fn parse_int(b: impl AsRef<[u8]>) -> usize {
    b.as_ref()
        .iter()
        .fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}
