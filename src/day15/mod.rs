use std::{cmp::Ordering, collections::BinaryHeap};

use anyhow::Result;
use fxhash::FxHashMap;
use ndarray::Array2;

use crate::{
    utils::{four_neighbors, single_digit_line},
    Runner,
};

type Coord = (usize, usize);

pub struct Day;

impl Runner for Day {
    type Input = Array2<usize>;
    type Output = usize;

    fn day() -> usize {
        15
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut v = input
            .lines()
            .map(str::as_bytes)
            .map(single_digit_line)
            .map(Result::unwrap)
            .map(|t| t.1)
            .map(Vec::into_iter)
            .flatten();
        Ok(Array2::from_shape_fn((height, width), |_| {
            v.next().unwrap()
        }))
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let shape = input.shape();
        Ok(astar(
            input,
            (0, 0),
            (shape[0] - 1, shape[1] - 1),
            &manhattan,
        ))
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let shape = input.shape();
        let input = Array2::from_shape_fn((shape[0] * 5, shape[1] * 5), |(x, y)| {
            let (grid_x, grid_y) = (
                (x as f64 / (shape[0] as f64)).floor() as usize,
                (y as f64 / (shape[1] as f64)).floor() as usize,
            );
            let (x, y) = (x % shape[0], y % shape[1]);
            let mut cost = input[(x, y)] + grid_x + grid_y;
            while cost > 9 {
                cost -= 9;
            }
            cost
        });

        let shape = input.shape();
        Ok(astar(
            &input,
            (0, 0),
            (shape[0] - 1, shape[1] - 1),
            &manhattan,
        ))
    }
}

pub fn astar<'a>(
    array: &'a Array2<usize>,
    start: Coord,
    end: Coord,
    heuristic: &Heuristic,
    // ) -> impl Iterator<Item = Coord> + 'a {
) -> usize {
    let shape = array.shape();
    let shape = (shape[0], shape[1]);
    let mut frontier = BinaryHeap::<Cell>::new();
    // let end = Cell::new(end, *array.get(end).unwrap());
    frontier.push(Cell::new(start, 0));
    let mut came_from = FxHashMap::<Coord, Option<Coord>>::default();
    let mut cost_so_far = FxHashMap::<Coord, usize>::default();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.loc == end {
            break;
        }
        four_neighbors(current.loc, shape).for_each(|next| {
            let new_cost = cost_so_far.get(&current.loc).unwrap() + array.get(next).unwrap();
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                frontier.push(Cell::new(next, new_cost + heuristic(next, end)));
                came_from.insert(next, Some(current.loc));
            }
        });
    }

    cost_so_far[&end]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    loc: Coord,
    cost: usize,
}

impl Cell {
    pub fn new(loc: Coord, cost: usize) -> Self {
        Self { loc, cost }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // MinHeap, then order by location
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Heuristic = dyn Fn(Coord, Coord) -> usize;

pub fn manhattan(a: Coord, b: Coord) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(40, Day::part1(&input)?);
        assert_eq!(315, Day::part2(&input)?);
        Ok(())
    }
}
