use std::{
    fmt::{Debug, Display},
    iter::once,
    ops::Deref,
};

use aoc_utils::{graph::four_neighbors, math::coord::Coord, Runner};
use hashbrown::HashMap;
use itertools::Itertools;
use miette::Result;
use pathfinding::directed::astar::astar_bag;

pub struct Day;

// Number Keypad
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

// Arrow Keypad
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberButton {
    N(usize),
    Gap,
    A,
}

impl NumberButton {
    const fn from(value: char) -> Self {
        match value {
            num @ '0'..='9' => Self::N(num as usize - '0' as usize),
            'A' => Self::A,
            _ => Self::Gap,
        }
    }
}

trait Keypad: Copy + std::hash::Hash + Eq {
    fn to_coord(self) -> Coord;

    fn gap() -> Coord;
    fn bounds() -> (usize, usize);

    fn paths(self, to: Self) -> Box<dyn Iterator<Item = Path>> {
        let Some((paths, _score)) = astar_bag(
            &self.to_coord(),
            |&coord| {
                four_neighbors(coord, Self::bounds())
                    .filter(|&c| c != Self::gap())
                    .map(|c| (c, 1))
            },
            |_| 1,
            |&c| c == to.to_coord(),
        ) else {
            panic!("No path found")
        };
        Box::new(paths.into_iter().map(|buttons| {
            if buttons.len() == 1 {
                return Path::from("A");
            }
            let mut path = Vec::new();
            log::debug!("Buttons: {buttons:?}");
            buttons.into_iter().reduce(|last, this| {
                path.push(ArrowButton::from_delta(this - last));
                this
            });
            path.push(ArrowButton::A);
            log::debug!("Path: {path:?}");
            Path { path }
        }))
    }
}

impl Keypad for NumberButton {
    fn to_coord(self) -> Coord {
        match self {
            NumberButton::N(7) => Coord(0, 0),
            NumberButton::N(8) => Coord(0, 1),
            NumberButton::N(9) => Coord(0, 2),
            NumberButton::N(4) => Coord(1, 0),
            NumberButton::N(5) => Coord(1, 1),
            NumberButton::N(6) => Coord(1, 2),
            NumberButton::N(1) => Coord(2, 0),
            NumberButton::N(2) => Coord(2, 1),
            NumberButton::N(3) => Coord(2, 2),
            NumberButton::N(0) => Coord(3, 1),
            NumberButton::A => Coord(3, 2),
            NumberButton::Gap => Coord(3, 0),
            NumberButton::N(_) => panic!("Invalid coord"),
        }
    }

    fn gap() -> Coord {
        Self::Gap.to_coord()
    }

    fn bounds() -> (usize, usize) {
        (4, 3)
    }
}

impl Debug for NumberButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for NumberButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N(n) => f.write_fmt(format_args!("{:1}", n % 10)),
            Self::A => f.write_str("A"),
            Self::Gap => f.write_str("."),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ArrowButton {
    Left,
    Down,
    Right,
    Up,
    A,
    Gap,
}

impl ArrowButton {
    const fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' | 'V' => Self::Down,
            '>' => Self::Right,
            '<' => Self::Left,
            'A' => Self::A,
            _ => Self::Gap,
        }
    }

    const fn from_delta(delta: Coord) -> Self {
        match delta {
            Coord(-1, 0) => ArrowButton::Up,
            Coord(1, 0) => ArrowButton::Down,
            Coord(0, -1) => ArrowButton::Left,
            Coord(0, 1) => ArrowButton::Right,
            _ => panic!("Invalid delta"),
        }
    }
}

impl Keypad for ArrowButton {
    fn to_coord(self) -> Coord {
        match self {
            ArrowButton::Gap => Coord(0, 0),
            ArrowButton::Up => Coord(0, 1),
            ArrowButton::A => Coord(0, 2),
            ArrowButton::Left => Coord(1, 0),
            ArrowButton::Down => Coord(1, 1),
            ArrowButton::Right => Coord(1, 2),
        }
    }

    fn gap() -> Coord {
        Self::Gap.to_coord()
    }

    fn bounds() -> (usize, usize) {
        (2, 3)
    }
}

impl Debug for ArrowButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for ArrowButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("<"),
            Self::Down => f.write_str("v"),
            Self::Right => f.write_str(">"),
            Self::Up => f.write_str("^"),
            Self::A => f.write_str("A"),
            Self::Gap => f.write_str("."),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<ArrowButton>,
}

impl Deref for Path {
    type Target = [ArrowButton];

    fn deref(&self) -> &Self::Target {
        self.path.as_slice()
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Path(")?;
        f.write_str(&self.path.iter().join(""))?;
        f.write_str(")")
    }
}

impl Path {
    fn from(s: &str) -> Self {
        Self {
            path: s.chars().map(ArrowButton::from).collect(),
        }
    }

    fn from_iter(buttons: &[ArrowButton]) -> Self {
        Self {
            path: buttons.to_vec(),
        }
    }
}

fn shortest_path<const N: usize>(
    code: &[NumberButton],
    cache: &mut HashMap<(usize, Path), usize>,
) -> usize {
    once(NumberButton::A)
        .chain(code.iter().copied())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = a.paths(b);
            paths
                .into_iter()
                .map(|path| shortest_path_arrow::<N>(&path, 1, cache))
                .min()
                .unwrap()
        })
        .sum::<usize>()
}

fn shortest_path_arrow<const N: usize>(
    code: &[ArrowButton],
    depth: usize,
    cache: &mut HashMap<(usize, Path), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, Path::from_iter(code))) {
        return cached;
    }

    if depth > N {
        return code.len();
    }

    let sum = once(ArrowButton::A)
        .chain(code.iter().copied())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = a.paths(b);
            if depth == N {
                paths.into_iter().map(|path| path.len()).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| shortest_path_arrow::<N>(&path, depth + 1, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>();
    cache.insert((depth, Path::from_iter(code)), sum);
    sum
}

fn value(buttons: &[NumberButton]) -> usize {
    buttons.iter().fold(0, |acc, button| {
        acc * match button {
            NumberButton::N(_) => 10,
            NumberButton::Gap => 1,
            NumberButton::A => 1,
        } + match button {
            NumberButton::N(i) => i % 10,
            NumberButton::Gap => 0,
            NumberButton::A => 0,
        }
    })
}

impl Runner for Day {
    type Input<'input> = Vec<Vec<NumberButton>>;

    #[rustfmt::skip]
    fn day() -> usize {
        21
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|line| line.chars().map(NumberButton::from).collect())
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut cache = HashMap::new();
        Ok(input
            .iter()
            .map(|combo| shortest_path::<2>(combo, &mut cache) * value(combo))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut cache = HashMap::new();
        Ok(input
            .iter()
            .map(|combo| shortest_path::<25>(combo, &mut cache) * value(combo))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                029A
                980A
                179A
                456A
                379A
            "};
            part1 = 126384;
            part2 = 154115708116294_usize;
    }

    prod_case! {
        part1 = 94284;
        part2 = 116821732384052;
    }

    #[test]
    fn press_1() {
        let _ = env_logger::try_init();
        let paths = NumberButton::A
            .paths(NumberButton::N(1))
            .collect::<HashSet<Path>>();
        assert_eq!(paths.len(), 2);
        assert_eq!(
            paths,
            HashSet::from_iter([Path::from("^<<A"), Path::from("<^<A")])
        );
    }

    #[test]
    fn press_9() {
        let _ = env_logger::try_init();
        let paths = NumberButton::A
            .paths(NumberButton::N(9))
            .collect::<HashSet<Path>>();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths, HashSet::from_iter([Path::from("^^^A")]));
    }

    #[test]
    fn press_7() {
        let _ = env_logger::try_init();
        let paths = NumberButton::A
            .paths(NumberButton::N(7))
            .collect::<HashSet<Path>>();
        assert_eq!(paths.len(), 9);
        assert_eq!(
            paths,
            HashSet::from_iter([
                Path::from("<^<^^A"),
                Path::from("^<<^^A"),
                Path::from("<^^<^A"),
                Path::from("^<^<^A"),
                Path::from("^^<<^A"),
                Path::from("<^^^<A"),
                Path::from("^<^^<A"),
                Path::from("^^<^<A"),
                Path::from("^^^<<A"),
            ])
        );
    }

    #[test]
    fn press_left_from_a() {
        let _ = env_logger::try_init();
        let paths = ArrowButton::A
            .paths(ArrowButton::Left)
            .collect::<HashSet<Path>>();
        assert_eq!(paths.len(), 2);
        assert_eq!(
            paths,
            HashSet::from_iter([Path::from("v<<A"), Path::from("<v<A"),])
        );
    }

    #[test]
    fn press_a() {
        let _ = env_logger::try_init();
        let paths = NumberButton::A
            .paths(NumberButton::A)
            .collect::<HashSet<Path>>();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths, HashSet::from_iter([Path::from("A"),]));
    }

    #[test]
    fn combo_029a() {
        let _ = env_logger::try_init();

        let mut cache = HashMap::new();
        let shortest = shortest_path::<0>(
            &[
                NumberButton::N(0),
                NumberButton::N(2),
                NumberButton::N(9),
                NumberButton::A,
            ],
            &mut cache,
        );
        assert_eq!(shortest, 12);
        // <A^A^^>AvvvA
    }

    #[test]
    fn one_robot_029a() {
        let _ = env_logger::try_init();

        let mut cache = HashMap::new();
        let shortest = shortest_path::<1>(
            &[
                NumberButton::N(0),
                NumberButton::N(2),
                NumberButton::N(9),
                NumberButton::A,
            ],
            &mut cache,
        );
        assert_eq!(shortest, 28);
        //    <   A ^ A ^^  > A  vvv  A
        // v<<A>>^A<A>A<AAv>A^A<vAAA^>A
    }

    #[test]
    fn two_robot_029a() {
        let _ = env_logger::try_init();

        let mut cache = HashMap::new();
        let shortest = shortest_path::<2>(
            &[
                NumberButton::N(0),
                NumberButton::N(2),
                NumberButton::N(9),
                NumberButton::A,
            ],
            &mut cache,
        );
        assert_eq!(shortest, 68);
    }

    #[test]
    fn two_robot_379a() {
        let _ = env_logger::try_init();

        let mut cache = HashMap::new();
        let shortest = shortest_path::<2>(
            &[
                NumberButton::N(3),
                NumberButton::N(7),
                NumberButton::N(9),
                NumberButton::A,
            ],
            &mut cache,
        );
        assert_eq!(shortest, 64);
    }
}
