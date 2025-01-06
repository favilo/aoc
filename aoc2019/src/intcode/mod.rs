use std::ops::{Index, IndexMut};

use aoc_utils::errors::ToMietteErr;
use miette::{miette, Diagnostic, Result};
use winnow::{
    ascii::{dec_uint, multispace0},
    combinator::{opt, repeat, terminated},
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Program end")]
    End,
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(usize),

    #[error("Input too short: {0} < {1}")]
    InputTooShort(usize, usize),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),

    Halt,
}

impl Op {
    pub fn parse(input: &[usize]) -> Result<(Self, usize), Error> {
        let (opcode, rest) = input
            .split_first()
            .ok_or(Error::InputTooShort(1, input.len()))?;
        match opcode {
            1 => Ok((Self::Add(rest[0], rest[1], rest[2]), 4)),
            2 => Ok((Self::Mul(rest[0], rest[1], rest[2]), 4)),
            99 => Ok((Self::Halt, 1)),
            _ => Err(Error::InvalidOpcode(*opcode)),
        }
    }

    fn execute(self, memory: &mut Program) -> Result<(), Error> {
        match self {
            Self::Add(a, b, c) => {
                log::debug!("{:?} + {:?} -> {c:?}", memory[a], memory[b]);
                memory[c] = memory[a] + memory[b]
            }
            Self::Mul(a, b, c) => {
                log::debug!("{:?} * {:?} -> {c:?}", memory[a], memory[b]);
                memory[c] = memory[a] * memory[b]
            }
            Self::Halt => {
                log::debug!("Halt");
                return Err(Error::End);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    memory: Vec<usize>,
    pc: usize,
}

impl Index<usize> for Program {
    type Output = usize;

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
            terminated(dec_uint::<_, usize, _>, (opt(","), multispace0)),
        )
        .map(|memory: Vec<_>| Program { memory, pc: 0 })
        .parse_next(input)
    }

    pub fn run(&mut self) -> Result<usize, Error> {
        loop {
            let res = self.step();
            if let Err(Error::End) = res {
                return Ok(self.memory[0]);
            }
            res?;
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        log::debug!("Program State: {self:?}");
        if self.pc >= self.memory.len() {
            return Err(Error::End);
        }
        let (op, pc_delta) = Op::parse(&self.memory[self.pc..])?;
        log::debug!("executing op: {op:?}");
        self.pc += pc_delta;
        log::debug!("pc: {:?}", self.pc);
        op.execute(self)
    }
}
