use anyhow::Result;
use itertools::iproduct;
use ndarray::{Array, Array2};
use std::cell::RefCell;
use std::iter::FromIterator;
use vec_map::VecMap;

use crate::Runner;

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Floor,
    Empty,
    Full,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => f.write_str("."),
            Self::Empty => f.write_str("L"),
            Self::Full => f.write_str("#"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    neighbors_cache: RefCell<VecMap<Vec<usize>>>,
}

impl Room {
    pub fn get(&self, (x, y): Coord) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[self.get_coord(x, y)])
    }

    pub fn get_coord(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn neighbors<F>(&self, (x, y): Coord, coord_fn: F) -> *const Vec<usize>
    where
        F: Fn(&Room, Coord) -> Vec<usize>,
    {
        let mut cache = self.neighbors_cache.borrow_mut();
        if cache.contains_key(self.get_coord(x, y)) {
            return &cache[self.get_coord(x, y)];
        }
        let v = coord_fn(self, (x, y));
        cache.insert(self.get_coord(x, y), v);
        &cache[self.get_coord(x, y)]
    }

    pub fn local_neighbors(&self, (x, y): Coord) -> Vec<usize> {
        iproduct!((-1isize..=1), (-1isize..=1))
            .filter(|&(a, b)| a != 0 || b != 0)
            .map(move |(a, b)| (a + x as isize, b + y as isize))
            .filter(move |&(x, y)| {
                x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
            })
            .map(|coord| self.get_coord(coord.0 as usize, coord.1 as usize))
            .collect()
    }

    pub fn extended_neighbors(&self, (x, y): Coord) -> Vec<usize> {
        iproduct!((-1isize..=1), (-1isize..=1))
            .filter(|&(a, b)| a != 0 || b != 0)
            .filter_map(move |(a, b)| {
                let coord = (1..)
                    .map(|i| (x as isize + (a * i), y as isize + (b * i)))
                    .take_while(|&(x, y)| {
                        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
                    })
                    .find(|&(x, y)| self.get((x as usize, y as usize)) != Some(Tile::Floor));
                coord.map(|coord| self.get_coord(coord.0 as usize, coord.1 as usize))
            })
            .collect()
    }

    pub fn step<F>(&mut self, coord_fn: F, crowd: usize) -> usize
    where
        F: Fn(&Room, Coord) -> Vec<usize> + Copy,
    {
        let changes = iproduct!((0..self.width), (0..self.height))
            .map(|coord| {
                let neighbors = self.neighbors(coord, coord_fn);
                (
                    coord,
                    self.get(coord),
                    unsafe { neighbors.as_ref().unwrap() }
                        .iter()
                        .map(|&coord| self.tiles[coord])
                        .filter(|&state| state == Tile::Full)
                        .count(),
                )
            })
            .filter_map(|(coord, state, count)| match (state, count) {
                (Some(Tile::Empty), 0) => Some((coord, Tile::Full)),
                (Some(Tile::Full), x) if x >= crowd => Some((coord, Tile::Empty)),
                _ => None,
            })
            .collect::<Vec<_>>();
        changes.iter().for_each(|&(coord, state)| {
            let coord = self.get_coord(coord.0, coord.1);
            self.tiles[coord] = state;
        });
        changes.len()
    }

    pub fn count_tiles(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|&t| *t == tile).count()
    }
}

#[allow(dead_code)]
fn print_room(room: &<Day11 as Runner>::Input) {
    let dim = room.dim();
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            print!("{}", room[(x, y)]);
        }
        println!();
    }
    println!();
}

pub struct Day11Unsafe;
pub struct Day11;

fn neighbors<'a>(
    room: &'a <Day11 as Runner>::Input,
    (x, y): (isize, isize),
    cache: &'a mut VecMap<Vec<(usize, usize)>>,
) -> &'a Vec<Coord> {
    let dim = room.dim();
    if cache.contains_key(x as usize * dim.0 + y as usize) {
        return &cache[&(x as usize * dim.0 + y as usize)];
    }
    let v = iproduct!((-1..=1), (-1..=1))
        .filter(|&(a, b)| a != 0 || b != 0)
        .map(move |(a, b)| (a + x, b + y))
        .filter(move |&(x, y)| x >= 0 && y >= 0 && x < dim.0 as isize && y < dim.1 as isize)
        .map(|coord| (coord.0 as usize, coord.1 as usize))
        .collect();
    cache.insert(x as usize * dim.0 + y as usize, v);
    &cache[&(x as usize * dim.0 + y as usize)]
}

fn extended_neighbors<'a>(
    room: &'a <Day11 as Runner>::Input,
    (x, y): (isize, isize),
    cache: &'a mut VecMap<Vec<(usize, usize)>>,
) -> &'a Vec<Coord> {
    let dim = room.dim();
    if cache.contains_key(x as usize * dim.0 + y as usize) {
        return &cache[&(x as usize * dim.0 + y as usize)];
    }
    let v = iproduct!((-1..=1), (-1..=1))
        .filter(|&(a, b)| a != 0 || b != 0)
        .filter_map(move |(a, b)| {
            let coord = (1..)
                .map(|i| (x + (a * i), y + (b * i)))
                .take_while(|&(x, y)| x >= 0 && y >= 0 && x < dim.0 as isize && y < dim.1 as isize)
                .find(|&(x, y)| room[(x as usize, y as usize)] != Tile::Floor);
            coord.map(|coord| (coord.0 as usize, coord.1 as usize))
        })
        .collect();
    cache.insert(x as usize * dim.0 + y as usize, v);
    &cache[&(x as usize * dim.0 + y as usize)]
}

fn step<F>(
    room: &mut <Day11 as Runner>::Input,
    neighbors_f: F,
    crowd: usize,
    cache: &mut VecMap<Vec<(usize, usize)>>,
) -> usize
where
    F: for<'a> Fn(
        &'a <Day11 as Runner>::Input,
        (isize, isize),
        &'a mut VecMap<Vec<(usize, usize)>>,
    ) -> &'a Vec<Coord>,
{
    let dim = room.dim();
    let changes = iproduct!((0..dim.0), (0..dim.1))
        .map(|coord| {
            (
                coord,
                room[coord],
                neighbors_f(&room, (coord.0 as isize, coord.1 as isize), cache)
                    .iter()
                    .map(|&coord| room[coord])
                    .filter(|&state| state == Tile::Full)
                    .count(),
            )
        })
        .filter_map(|(coord, state, count)| match (state, count) {
            (Tile::Empty, 0) => Some((coord, Tile::Full)),
            (Tile::Full, x) if x >= crowd => Some((coord, Tile::Empty)),
            _ => None,
        })
        .collect::<Vec<_>>();
    changes.iter().for_each(|&(coord, state)| {
        room[coord] = state;
    });
    changes.len()
}

fn parse_room(input: &str) -> Result<<Day11 as Runner>::Input> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let iter = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            'L' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Full,
            _ => unreachable!(),
        });
    Ok(Array::from_iter(iter)
        .into_shape((height, width))?
        .reversed_axes())
}

fn parse_room_unsafe(input: &str) -> Result<<Day11Unsafe as Runner>::Input> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let iter = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            'L' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Full,
            _ => unreachable!(),
        });
    Ok(Room {
        width,
        height,
        tiles: iter.collect(),
        neighbors_cache: RefCell::new(VecMap::new()),
    })
}

impl Runner for Day11 {
    type Input = Array2<Tile>;
    type Output = usize;

    fn day() -> usize {
        11
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        parse_room(input)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut cache = VecMap::new();
        let mut room = input.clone();
        while 0 != step(&mut room, neighbors, 4, &mut cache) {
            // print_room(&room);
        }
        Ok(room.into_iter().filter(|&t| *t == Tile::Full).count())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut cache = VecMap::new();
        let mut room = input.clone();
        while 0 != step(&mut room, extended_neighbors, 5, &mut cache) {
            // print_room(&room);
        }
        Ok(room.into_iter().filter(|&t| *t == Tile::Full).count())
    }
}

impl Runner for Day11Unsafe {
    type Input = Room;
    type Output = usize;

    fn day() -> usize {
        11
    }

    fn comment() -> &'static str {
        "UNSAFE"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        parse_room_unsafe(input)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut room = input.clone();
        while 0 != room.step(Room::local_neighbors, 4) {
            // print_room(&room);
        }
        Ok(room.count_tiles(Tile::Full))
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut room: Room = input.clone();
        while 0 != room.step(Room::extended_neighbors, 5) {
            // print_room(&room);
        }
        Ok(room.count_tiles(Tile::Full))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample2() -> Result<()> {
        let input = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";

        let input = Day11::get_input(input)?;
        print_room(&input);
        let expected = "#.#L.L#.##\n\
                        #LLL#LL.L#\n\
                        L.#.L..#..\n\
                        #L##.##.L#\n\
                        #.#L.LL.LL\n\
                        #.#L#L#.##\n\
                        ..L.L.....\n\
                        #L#L##L#L#\n\
                        #.LLLLLL.L\n\
                        #.#L#L#.##";
        let mut room = input.clone();
        while step(&mut room, neighbors, 4, &mut VecMap::new()) != 0 {
            // println!("{:?}", room);
        }
        let expected = parse_room(expected)?;

        let dim = room.dim();
        for y in 0..dim.1 {
            print!("{}", room.slice(ndarray::s![.., y]));
            println!("\t{}", expected.slice(ndarray::s![.., y]));
        }
        // println!("{}", room);
        // println!("{}", expected);

        assert_eq!(room, expected);

        let output = Day11::part1(&input)?;
        assert_eq!(37, output);
        assert_eq!(26, Day11::part2(&input)?);
        Ok(())
    }

    #[test]
    fn all_steps() -> Result<()> {
        let input = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let mut room = parse_room(input)?;
        let round1 = "#.##.##.##\n\
                      #######.##\n\
                      #.#.#..#..\n\
                      ####.##.##\n\
                      #.##.##.##\n\
                      #.#####.##\n\
                      ..#.#.....\n\
                      ##########\n\
                      #.######.#\n\
                      #.#####.##";
        let round1 = parse_room(round1)?;
        step(&mut room, neighbors, 4, &mut VecMap::new());

        let dim = room.dim();
        for y in 0..dim.1 {
            print!("{}", room.slice(ndarray::s![.., y]));
            println!("\t{}", round1.slice(ndarray::s![.., y]));
        }
        print!("{:?}", room);

        assert_eq!(room, round1);

        Ok(())
    }

    #[test]
    fn all_steps2() -> Result<()> {
        let input = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let mut room = parse_room(input)?;
        let round1 = "#.##.##.##\n\
                      #######.##\n\
                      #.#.#..#..\n\
                      ####.##.##\n\
                      #.##.##.##\n\
                      #.#####.##\n\
                      ..#.#.....\n\
                      ##########\n\
                      #.######.#\n\
                      #.#####.##";
        let round1 = parse_room(round1)?;
        step(&mut room, extended_neighbors, 5, &mut VecMap::new());
        assert_eq!(room, round1);

        let round2 = "#.LL.LL.L#\n\
                      #LLLLLL.LL\n\
                      L.L.L..L..\n\
                      LLLL.LL.LL\n\
                      L.LL.LL.LL\n\
                      L.LLLLL.LL\n\
                      ..L.L.....\n\
                      LLLLLLLLL#\n\
                      #.LLLLLL.L\n\
                      #.LLLLL.L#";
        let round2 = parse_room(round2)?;
        step(&mut room, extended_neighbors, 5, &mut VecMap::new());

        let dim = room.dim();
        for y in 0..dim.1 {
            print!("{}", room.slice(ndarray::s![.., y]));
            println!("\t{}", round1.slice(ndarray::s![.., y]));
        }
        print!("{:?}", room);
        assert_eq!(room, round2);

        Ok(())
    }
}
