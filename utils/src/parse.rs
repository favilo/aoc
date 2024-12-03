pub fn parse_int(b: &[u8]) -> usize {
    b.iter().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}
