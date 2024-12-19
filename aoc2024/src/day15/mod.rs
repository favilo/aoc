use std::{cell::RefCell, char::ParseCharError, collections::BinaryHeap, rc::Rc, str::FromStr};

use aoc_utils::math::coord::Coord;
use hashbrown::HashMap;
use miette::{IntoDiagnostic, Result};
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Block,
    DefaultTerminal,
};
use winnow::{
    ascii::{line_ending, multispace0},
    combinator::{alt, fail, repeat, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Location, Stream, StreamIsPartial},
    token::take,
    Located, PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DepthPair(usize, Coord, Object);

impl PartialOrd for DepthPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for DepthPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    #[default]
    Space,
    Wall,
    Box,
    Bot,
    OpenBox,
    CloseBox,
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => f.write_str("."),
            Self::Wall => f.write_str("#"),
            Self::Box => f.write_str("O"),
            Self::Bot => f.write_str("@"),
            Self::OpenBox => f.write_str("["),
            Self::CloseBox => f.write_str("]"),
        }
    }
}

impl FromStr for Object {
    type Err = ParseCharError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match char::from_str(s)? {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Bot),
            _ => Err(char::from_str("bad").unwrap_err()),
        }
    }
}

impl Object {
    pub fn opposite(self) -> Self {
        match self {
            Self::Space => Self::Space,
            Self::Wall => Self::Wall,
            Self::Box => Self::Box,
            Self::Bot => Self::Space,
            Self::OpenBox => Self::CloseBox,
            Self::CloseBox => Self::OpenBox,
        }
    }

    pub fn pair_coord(self, coord: Coord) -> Coord {
        match self {
            Self::OpenBox => coord + Coord(1, 0),
            Self::CloseBox => coord + Coord(-1, 0),
            _ => coord,
        }
    }

    pub fn blocks(self) -> bool {
        matches!(self, Self::Wall)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ParseCharError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match char::from_str(s)? {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(char::from_str("bad").unwrap_err()),
        }
    }
}

impl Dir {
    fn to_coord(self) -> Coord {
        match self {
            Self::Up => Coord(0, -1),
            Self::Down => Coord(0, 1),
            Self::Left => Coord(-1, 0),
            Self::Right => Coord(1, 0),
        }
    }
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Up => write!(f, "^"),
            Dir::Down => write!(f, "v"),
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Warehouse {
    width: usize,
    height: usize,
    bot: Coord,
    boxes: HashMap<Coord, Object>,
    instructions: Vec<Dir>,

    ip: usize,
    messages: Vec<String>,
}

impl std::fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Warehouse")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("bot", &self.bot)
            .field_with("instructions", |f| {
                f.write_str(&format!("{:?}", self.instructions))
            })
            .field("ip", &self.ip)
            .field(
                "current_dir",
                &self
                    .instructions
                    .get(self.ip)
                    .map(|d| format!("{d:?}"))
                    .unwrap_or_default(),
            )
            .field_with("boxes", |f| {
                f.write_str("\n")?;
                (0..self.height).try_for_each(|j| {
                    let mut s = String::with_capacity(self.width);
                    (0..self.width).for_each(|i| {
                        s.push_str(&format!(
                            "{:?}",
                            self.boxes
                                .get(&Coord(i as isize, j as isize))
                                .unwrap_or(&Object::Space)
                        ))
                    });
                    s.push('\n');
                    f.write_str(&s)
                })
            })
            .finish()
    }
}

impl Warehouse {
    pub fn run(&mut self, mut terminal: Option<&mut DefaultTerminal>) -> Result<()> {
        for _ in 0..self.instructions.len() {
            self.step();
            if let Some(terminal) = terminal.as_mut() {
                // let start = Instant::now();
                loop {
                    terminal
                        .draw(|f| {
                            let msg_percent = 40;
                            let layout = Layout::default()
                                .direction(Direction::Vertical)
                                .margin(1)
                                .constraints(
                                    [
                                        Constraint::Percentage(100 - msg_percent),
                                        Constraint::Percentage(msg_percent),
                                    ]
                                    .as_ref(),
                                )
                                .split(f.area());
                            f.render_widget(self.to_widget(), layout[0]);
                            f.render_widget(self.message_widget(), layout[1]);
                        })
                        .into_diagnostic()?;
                    if let Event::Key(key_event) = event::read().into_diagnostic()? {
                        match key_event.code {
                            event::KeyCode::Char('q') => return Ok(()),
                            event::KeyCode::Char(' ') => break,
                            d @ (event::KeyCode::Left
                            | event::KeyCode::Right
                            | event::KeyCode::Up
                            | event::KeyCode::Down) => {
                                self.instructions.clear();
                                self.ip = 0;
                                self.instructions.push(match d {
                                    event::KeyCode::Left => Dir::Left,
                                    event::KeyCode::Right => Dir::Right,
                                    event::KeyCode::Up => Dir::Up,
                                    event::KeyCode::Down => Dir::Down,
                                    _ => unreachable!(),
                                });
                                break;
                            }
                            _ => {}
                        }
                    }
                    // if start.elapsed() > Duration::from_millis(100) {
                    //     break;
                    // }
                }
            }
        }
        Ok(())
    }

    pub fn step(&mut self) {
        let dir = self.instructions[self.ip];
        self.ip += 1;
        self.messages.push("Stepping".to_string());
        self.move_object(self.bot, Object::Bot, dir);
    }

    pub fn to_widget(&self) -> ratatui::widgets::Paragraph<'_> {
        let t = Text::raw(format!("{self:#?}"));
        ratatui::widgets::Paragraph::new(t)
    }

    pub fn message_widget(&self) -> ratatui::widgets::Paragraph<'_> {
        let mut messages = Vec::with_capacity(self.messages.len());
        messages.extend(
            self.messages
                .iter()
                .enumerate()
                .skip(isize::max(self.messages.len() as isize - 27, 0) as usize)
                .map(|(i, l)| {
                    Line::from(vec![
                        Span::styled(i.to_string(), Color::White),
                        Span::raw(" - "),
                        Span::styled(l.clone(), Color::Green),
                    ])
                }),
        );
        let t = Text::from(messages);
        ratatui::widgets::Paragraph::new(t).block(
            Block::bordered()
                .title("Messages")
                .border_style(Style::default().fg(Color::Red)),
        )
    }

    fn move_object(&mut self, here: Coord, this: Object, dir: Dir) -> bool {
        self.messages
            .push(format!("Found  `{this:?}` @ {here:?} going {dir:?}"));

        let mut to_move = Default::default();
        let blocked = self.is_blocked(0, here, this, dir, &mut to_move);
        if !blocked {
            // self.messages.push(format!("Moving {to_move:?}"));
            while let Some(DepthPair(_i, coord, object)) = to_move.pop() {
                // self.messages
                //     .push(format!("Depth {i}: Moving {coord:?} to {object:?}"));
                self.boxes.insert(coord, object);
            }
            // Clear out my spot, to make room for the recursive calls
            self.boxes.insert(here, Object::Space);
            if this == Object::Bot {
                self.bot = here + dir.to_coord();
                self.boxes.insert(self.bot, Object::Bot);
            }
        }

        blocked
    }

    fn is_blocked(
        &mut self,
        depth: usize,
        here: Coord,
        this: Object,
        dir: Dir,
        to_move: &mut BinaryHeap<DepthPair>,
    ) -> bool {
        // if this.blocks() {
        //     self.messages.push(format!("Blocked `{here:?}`"));
        //     return true;
        // }
        let spaces = String::from_iter(std::iter::repeat(' ').take(depth * 2));

        let there = here + dir.to_coord();
        let Some(&that) = self.boxes.get(&there) else {
            self.messages.push(format!("{spaces}Empty `{there:?}`"));
            return true;
        };

        match (dir, that) {
            (_, Object::Wall) => {
                self.messages.push(format!("{spaces}Blocked `{that:?}`"));
                true
            }
            (_, Object::Bot | Object::Space) => {
                self.messages.push(format!(
                    "{spaces}Can move: `{this:?}` to {there:?} was `{that:?}`",
                ));
                // Clear here
                to_move.push(DepthPair(depth + 1, here, Object::Space));
                // Move to there
                to_move.push(DepthPair(depth, there, this));
                false
            }
            (Dir::Up | Dir::Down, Object::OpenBox | Object::CloseBox) => {
                self.messages
                    .push(format!("{spaces}Checking wide box: `{that:?}` {there:?}"));
                let first_blocked = self.is_blocked(depth + 1, there, that, dir, to_move);

                self.messages.push(format!(
                    "{spaces}Checking wide box: `{:?}` {:?}",
                    that.opposite(),
                    that.pair_coord(there)
                ));
                let other_blocked = self.is_blocked(
                    depth + 1,
                    that.pair_coord(there),
                    that.opposite(),
                    dir,
                    to_move,
                );
                let blocked = first_blocked || other_blocked;
                if !(blocked) {
                    self.messages.push(format!(
                        "{spaces}Can move: `{this:?}` from {here:?} to {there:?} and `{:?}` from {:?} to {:?}",
                        this.opposite(),
                        this.pair_coord(here),
                        that.pair_coord(there)
                    ));
                    // Clear here and pair
                    to_move.push(DepthPair(depth + 1, here, Object::Space));
                    to_move.push(DepthPair(depth + 1, this.pair_coord(here), Object::Space));
                    // Move to there
                    to_move.push(DepthPair(depth, there, this));
                    to_move.push(DepthPair(depth, this.pair_coord(there), this.opposite()));
                }
                blocked
            }
            (_, Object::Box | Object::OpenBox | Object::CloseBox) => {
                self.messages.push(format!("{spaces}box: `{that:?}`"));
                let blocked = self.is_blocked(depth + 1, there, that, dir, to_move);
                if !blocked {
                    to_move.push(DepthPair(depth + 1, here, Object::Space));
                    to_move.push(DepthPair(depth, there, this));
                }
                blocked
            }
        }
    }

    pub fn move_wide_object(&mut self, here: Coord, this: Object, dir: Dir) -> bool {
        let there = here + dir.to_coord();
        let Some(&that) = self.boxes.get(&there) else {
            return false;
        };
        let there2 = this.pair_coord(there);
        let Some(&that2) = self.boxes.get(&there2) else {
            return false;
        };
        self.messages.push(format!(
            "Found [`{that:?}`, `{that2:?}`] at [{there:?},{there2:?}]"
        ));

        if that.blocks() || that2.blocks() {
            self.messages.push("Blocked".to_string());
            return false;
        }

        if self.move_object(there, that, dir) && self.move_object(there2, that2, dir) {
            self.boxes.insert(there, this);
            self.boxes.insert(there2, this.opposite());
            self.boxes.insert(here, Object::Space);
            self.boxes.insert(this.pair_coord(here), Object::Space);
            true
        } else {
            false
        }
    }

    pub fn gps_sum(&self) -> usize {
        self.boxes
            .iter()
            .filter(|&(_, v)| matches!(v, Object::Box | Object::OpenBox))
            .map(|(&Coord(i, j), _)| 100 * j as usize + i as usize)
            .sum()
    }

    pub fn widen(&self) -> Self {
        let boxes: HashMap<_, _> = self
            .boxes
            .iter()
            .flat_map(|(&Coord(i, j), &b)| match b {
                // TODO: Do some special handling to take care of double wide boxes.
                Object::Box => vec![
                    (Coord(i * 2, j), Object::OpenBox),
                    (Coord(i * 2 + 1, j), Object::CloseBox),
                ],
                Object::Space | Object::Wall => {
                    vec![(Coord(i * 2, j), b), (Coord(i * 2 + 1, j), b)]
                }
                Object::Bot => vec![
                    (Coord(i * 2, j), Object::Bot),
                    (Coord(i * 2 + 1, j), Object::Space),
                ],
                _ => vec![],
            })
            .collect();
        Self {
            width: self.width * 2,
            boxes,
            bot: Coord(self.bot.0 * 2, self.bot.1),
            ..self.clone()
        }
    }

    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str> + Location,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        (Self::map, Self::instructions)
            .map(|((width, height, bot, boxes), instructions)| Self {
                width,
                height,
                bot,
                boxes,
                instructions,
                ip: 0,
                messages: Vec::new(),
            })
            .parse_next(input)
    }

    fn map<S>(input: &mut S) -> PResult<(usize, usize, Coord, HashMap<Coord, Object>)>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str> + Location,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        seq!(
            // #   O   #
            repeat(1.., Self::box_line),
            // Blank line
            _: line_ending,
        )
        .map(|(boxes,): (Vec<Vec<Object>>,)| {
            let height = boxes.len();
            let width = boxes[0].len();
            let bot = Rc::new(RefCell::new(None));
            let set = boxes
                .iter()
                .enumerate()
                .flat_map(|(j, line)| {
                    let inner_bot = bot.clone();
                    line.iter()
                        .copied()
                        .enumerate()
                        .inspect(move |&(i, obj)| {
                            if obj == Object::Bot {
                                *inner_bot.borrow_mut() = Some(Coord(i as isize, j as isize));
                            }
                        })
                        .map(move |(i, obj)| (Coord(i as isize, j as isize), obj))
                })
                .collect();
            let bot = bot.borrow().expect("no bot found");
            (width, height, bot, set)
        })
        .parse_next(input)
    }

    fn instructions<S>(input: &mut S) -> PResult<Vec<Dir>>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str> + Location,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        seq!(repeat(
            1..,
            terminated(take(1usize), multispace0)
                .try_map(|s: <S as Stream>::Slice| Dir::from_str(s.as_ref()))
        ),)
        .map(|(instructions,)| instructions)
        .parse_next(input)
    }

    fn box_line<S>(input: &mut S) -> PResult<Vec<Object>>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str> + Location,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        seq!(
            repeat(1.., Self::object),
            _: line_ending,
        )
        .map(|t| t.0)
        .parse_next(input)
    }

    fn object<S>(input: &mut S) -> PResult<Object>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str> + Location,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        alt((
            take(1usize).try_map(|c: <S as Stream>::Slice| Object::from_str(c.as_ref())),
            fail,
        ))
        .parse_next(input)
    }
}

impl Runner for Day {
    type Input<'input> = Warehouse;

    fn day() -> usize {
        15
    }

    fn comment() -> &'static str {
        "Run with `cargo run -F day15_ratatui --release -- -d 15` to get a simple UI"
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Warehouse::parser.parse(Located::new(input)).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut warehouse = input.clone();
        warehouse.run(None)?;
        log::debug!("Warehouse: {warehouse:?}");
        Ok(warehouse.gps_sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        #[cfg(feature = "day15_ratatui")]
        let mut terminal = ratatui::init();
        #[cfg(feature = "day15_ratatui")]
        terminal.clear().into_diagnostic()?;

        let warehouse = input.clone();
        let mut warehouse = warehouse.widen();
        #[cfg(feature = "day15_ratatui")]
        warehouse.run(Some(&mut terminal))?;
        #[cfg(not(feature = "day15_ratatui"))]
        warehouse.run(None)?;

        #[cfg(feature = "day15_ratatui")]
        ratatui::restore();
        Ok(warehouse.gps_sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        small1 =>
            input = "\
                ########\n\
                #..O.O.#\n\
                ##@.O..#\n\
                #...O..#\n\
                #.#.O..#\n\
                #...O..#\n\
                #......#\n\
                ########\n\
                \n\
                <^^>>>vv<v>>v<<\n\
                ";
            part1 = 2028;
            part2 = 1751;
    }

    sample_case! {
        sample1 =>
            input = "\
                ##########\n\
                #..O..O.O#\n\
                #......O.#\n\
                #.OO..O.O#\n\
                #..O@..O.#\n\
                #O#..O...#\n\
                #O..O..O.#\n\
                #.OO.O.OO#\n\
                #....O...#\n\
                ##########\n\
                \n\
                <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
            ";
            part1 = 10092;
            part2 = 9021;
    }

    prod_case! {
        part1 = 1465523;
        part2 = 1471049;
    }
}
