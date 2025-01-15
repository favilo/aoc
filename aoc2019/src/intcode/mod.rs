use std::ops::{Index, IndexMut};

use aoc_utils::errors::ToMietteErr;
use miette::{miette, Diagnostic, Result};
use winnow::{
    ascii::{dec_int, multispace0},
    combinator::{opt, repeat, terminated},
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Program end")]
    End,

    #[error("Invalid opcode: {0}")]
    InvalidOpcode(isize),

    #[error("Input too short: {0} < {1}")]
    InputTooShort(usize, usize),

    #[error("Invalid mode: {0}")]
    InvalidMode(isize),

    #[error("Invalid destination: {0:?}")]
    InvalidDestination(Location),

    #[error("Invalid position: {0}")]
    InvalidPosition(isize),

    #[error("Not enough inputs")]
    NotEnoughInputs,
}

impl ToMietteErr for Error {
    fn to_miette(self) -> miette::Report {
        miette!("IntCode Error: {self}")
    }
}

impl Diagnostic for Error {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn severity(&self) -> Option<miette::Severity> {
        None
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        None
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        None
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        None
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Position,
    Immediate,
}

impl Mode {
    fn parse(input: isize) -> Result<[Self; 3], Error> {
        let input = input / 100;
        let (a, b, c) = (input % 10, (input / 10) % 10, (input / 100) % 10);
        Ok([Self::new(a)?, Self::new(b)?, Self::new(c)?])
    }

    fn new(mode: isize) -> Result<Self, Error> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(Error::InvalidMode(mode)),
        }
    }

    fn to_location(self, input: isize) -> Result<Location, Error> {
        match self {
            Self::Position => {
                if input < 0 {
                    return Err(Error::InvalidPosition(input));
                }
                Ok(Location::Position(input as usize))
            }
            Self::Immediate => Ok(Location::Immediate(input)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Position(usize),
    Immediate(isize),
}

impl Location {
    fn value(self, program: &Program) -> isize {
        match self {
            Self::Position(i) => program[i],
            Self::Immediate(i) => i,
        }
    }

    fn set_value(self, program: &mut Program, value: isize) -> Result<(), Error> {
        match self {
            Self::Position(i) => {
                *program.index_mut(i) = value;
                Ok(())
            }
            Self::Immediate(_) => Err(Error::InvalidDestination(self)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add(Location, Location, Location),
    Mul(Location, Location, Location),

    Input(Location),
    Output(Location),

    JumpIfTrue(Location, Location),
    JumpIfFalse(Location, Location),
    LessThan(Location, Location, Location),
    Equals(Location, Location, Location),

    Halt,
}

type ConstructorFn = dyn Fn([Mode; 3], &[isize]) -> Result<Operation, Error>;

impl Operation {
    pub fn parse(input: &[isize]) -> Result<Self, Error> {
        let (&opcode, rest) = input
            .split_first()
            .ok_or(Error::InputTooShort(1, input.len()))?;
        let (opcode, modes) = (opcode % 100, Mode::parse(opcode)?);
        let constructor: Result<&ConstructorFn, Error> = match opcode {
            1 => Ok(&Self::add),
            2 => Ok(&Self::mul),

            3 => Ok(&Self::input),
            4 => Ok(&Self::output),

            5 => Ok(&Self::jump_if_true),
            6 => Ok(&Self::jump_if_false),
            7 => Ok(&Self::less_than),
            8 => Ok(&Self::equals),

            99 => Ok(&Self::halt),
            _ => Err(Error::InvalidOpcode(opcode)),
        };
        constructor?(modes, rest)
    }

    fn add(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b, c) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
            modes[2].to_location(input[2])?,
        );
        Ok(Self::Add(a, b, c))
    }

    fn mul(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b, c) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
            modes[2].to_location(input[2])?,
        );
        Ok(Self::Mul(a, b, c))
    }

    fn input(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let a = modes[0].to_location(input[0])?;
        Ok(Self::Input(a))
    }

    fn output(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let a = modes[0].to_location(input[0])?;
        Ok(Self::Output(a))
    }

    fn jump_if_true(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
        );
        Ok(Self::JumpIfTrue(a, b))
    }

    fn jump_if_false(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
        );
        Ok(Self::JumpIfFalse(a, b))
    }

    fn less_than(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b, c) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
            modes[2].to_location(input[2])?,
        );
        Ok(Self::LessThan(a, b, c))
    }

    fn equals(modes: [Mode; 3], input: &[isize]) -> Result<Self, Error> {
        let (a, b, c) = (
            modes[0].to_location(input[0])?,
            modes[1].to_location(input[1])?,
            modes[2].to_location(input[2])?,
        );
        Ok(Self::Equals(a, b, c))
    }

    fn halt(_modes: [Mode; 3], _input: &[isize]) -> Result<Self, Error> {
        Ok(Self::Halt)
    }

    fn execute(self, program: &mut Program, inputs: &mut &[isize]) -> Result<(), Error> {
        match self {
            Self::Add(a, b, c) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("{a} + {b} -> {c:?}");
                c.set_value(program, a + b)?;
            }
            Self::Mul(a, b, c) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("{a} * {b} -> {c:?}");
                c.set_value(program, a * b)?;
            }
            Self::Input(a) => {
                let (&value, new_inputs) = inputs.split_first().ok_or(Error::NotEnoughInputs)?;
                *inputs = new_inputs;
                log::debug!("Input {value} -> {a:?}");
                a.set_value(program, value)?;
            }
            Self::Output(a) => {
                let value = a.value(program);
                log::debug!("Output {value}");
                program.output(value)?;
            }

            Self::JumpIfTrue(a, b) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("JumpIfTrue {a} {b}");
                if a != 0 {
                    program.pc = b as usize;
                }
            }
            Self::JumpIfFalse(a, b) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("JumpIfFalse {a} {b}");
                if a == 0 {
                    program.pc = b as usize;
                }
            }
            Self::LessThan(a, b, c) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("LessThan {a} {b} -> {c:?}");
                c.set_value(program, if a < b { 1 } else { 0 })?;
            }
            Self::Equals(a, b, c) => {
                let a = a.value(program);
                let b = b.value(program);
                log::debug!("Equals {a} {b} -> {c:?}");
                c.set_value(program, if a == b { 1 } else { 0 })?;
            }
            Self::Halt => {
                log::debug!("Halt");
                return Err(Error::End);
            }
        }
        Ok(())
    }

    fn delta(&self) -> usize {
        match self {
            Self::Halt => 1,
            Self::Input(_) | Self::Output(_) => 2,
            Self::JumpIfTrue(_, _) | Self::JumpIfFalse(_, _) => 3,
            Self::Add(_, _, _)
            | Self::Mul(_, _, _)
            | Self::LessThan(_, _, _)
            | Self::Equals(_, _, _) => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    memory: Vec<isize>,

    outputs: Vec<isize>,
    pc: usize,
}

impl Index<usize> for Program {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        self.memory.get(index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        &mut self.memory[index]
    }
}

impl Program {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        repeat(
            1..,
            terminated(dec_int::<_, isize, _>, (opt(","), multispace0)),
        )
        .map(|memory: Vec<_>| Program {
            memory,

            outputs: Vec::new(),
            pc: 0,
        })
        .parse_next(input)
    }

    pub fn run(&mut self, inputs: &mut &[isize]) -> Result<&[isize], Error> {
        loop {
            let res = self.step(inputs);
            if let Err(Error::End) = res {
                return Ok(&self.outputs);
            }
            res?;
        }
    }

    pub fn step(&mut self, inputs: &mut &[isize]) -> Result<(), Error> {
        log::debug!("Program State: {self:?}");
        if self.pc >= self.memory.len() {
            return Err(Error::End);
        }
        let op = Operation::parse(&self.memory[self.pc..])?;
        log::debug!("executing op: {op:?}");
        self.pc += op.delta();
        log::debug!("pc: {:?}", self.pc);
        op.execute(self, inputs)
    }

    pub fn output(&mut self, value: isize) -> Result<(), Error> {
        self.outputs.push(value);
        Ok(())
    }
}
