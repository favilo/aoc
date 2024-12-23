use num::cast::AsPrimitive;
use num::Num;

pub fn four_neighbors<I, N>(idx: I, shape: (N, N)) -> impl Iterator<Item = I>
where
    N: Num + AsPrimitive<isize> + Copy + PartialOrd<N>,
    isize: AsPrimitive<N>,
    I: Into<(N, N)> + From<(N, N)> + Copy,
{
    four_neighbors_no_filter(idx.into())
        .filter(|&(x, y)| x >= N::zero() && y >= N::zero())
        .filter(move |&(x, y)| x < shape.0 && y < shape.1)
        .map(I::from)
}

pub fn four_neighbors_no_filter<N>(idx: (N, N)) -> impl Iterator<Item = (N, N)>
where
    N: Num + AsPrimitive<isize> + Copy,
    isize: AsPrimitive<N>,
{
    [
        (idx.0.as_() - 1, idx.1.as_()),
        (idx.0.as_(), idx.1.as_() - 1),
        (idx.0.as_() + 1, idx.1.as_()),
        (idx.0.as_(), idx.1.as_() + 1),
    ]
    .into_iter()
    .map(|(x, y)| (x.as_(), y.as_()))
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
