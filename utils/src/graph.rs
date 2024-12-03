pub fn four_neighbors(
    idx: (usize, usize),
    shape: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [
        (idx.0 as isize - 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize - 1),
        (idx.0 as isize + 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| x >= 0 && y >= 0)
    .filter(move |&(x, y)| x < shape.0 as isize && y < shape.1 as isize)
    .map(|(x, y)| (x as usize, y as usize))
}

pub fn eight_neighbors(
    idx: (usize, usize),
    shape: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    itertools::iproduct!(-1..=1isize, -1..=1isize)
        .filter(move |&(x, y)| x != 0 || y != 0)
        .filter(move |&(x, y)| idx.0 as isize + x >= 0 && idx.1 as isize + y >= 0)
        .filter(move |&(x, y)| {
            idx.0 as isize + x < (shape.0 as isize) && idx.1 as isize + y < (shape.1 as isize)
        })
        .map(move |(x, y)| ((idx.0 as isize + x) as usize, (idx.1 as isize + y) as usize))
}

pub fn six_neighbors(idx: [isize; 3]) -> impl Iterator<Item = [isize; 3]> {
    [
        [idx[0] - 1, idx[1], idx[2]],
        [idx[0], idx[1] - 1, idx[2]],
        [idx[0], idx[1], idx[2] - 1],
        [idx[0] + 1, idx[1], idx[2]],
        [idx[0], idx[1] + 1, idx[2]],
        [idx[0], idx[1], idx[2] + 1],
    ]
    .into_iter()
}
